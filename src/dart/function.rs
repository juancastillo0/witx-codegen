use std::io::Write;

use super::*;

impl DartGenerator {
    pub fn define_func<T: Write>(
        w: &mut PrettyWriter<T>,
        module_name: &str,
        func_witx: &witx::Function,
    ) -> Result<(), Error> {
        assert_eq!(func_witx.abi, witx::Abi::Preview1);
        let name = func_witx.name.as_str().to_string();
        let params_witx = &func_witx.params;
        let mut params = vec![];
        for param_witx in params_witx {
            let param_name = param_witx.name.as_str();
            let param_type = ASType::from(&param_witx.tref);
            params.push((param_name.to_string(), param_type));
        }

        let results_witx = &func_witx.results;
        assert_eq!(results_witx.len(), 1);
        let result_witx = &results_witx[0];
        let result = ASType::from(&result_witx.tref);
        let result = match result {
            ASType::Result(result) => result,
            _ => unreachable!(),
        };

        let ok_type = result.ok_type.clone();

        let docs = &func_witx.docs;
        if !docs.is_empty() {
            Self::write_docs(w, docs)?;
        }

        let mut params_decomposed = vec![];

        for param in &params {
            let mut decomposed = param.1.decompose(&param.0, false);
            params_decomposed.append(&mut decomposed);
        }

        let mut results = vec![];
        // A tuple in a result is expanded into additional parameters, transformed to
        // pointers
        if let ASType::Tuple(tuple_members) = ok_type.as_ref().leaf() {
            for (i, tuple_member) in tuple_members.iter().enumerate() {
                let name = format!("result{}_ptr", i);
                results.push((name, tuple_member.type_.clone()));
            }
        } else {
            let name = "result_ptr";
            results.push((name.to_string(), ok_type));
        }
        let mut results_decomposed = vec![];
        for result in &results {
            let mut decomposed = result.1.decompose(&result.0, true);
            results_decomposed.append(&mut decomposed);
        }

        Self::define_func_raw(
            w,
            module_name,
            &name,
            &params_decomposed,
            &results_decomposed,
            &result,
        )?;

        let signature_witx = func_witx.wasm_signature(witx::CallMode::DefinedImport);
        let params_count_witx = signature_witx.params.len() + signature_witx.results.len();
        assert_eq!(
            params_count_witx,
            params_decomposed.len() + results_decomposed.len() + 1
        );

        Ok(())
    }

    fn define_func_raw<T: Write>(
        w: &mut PrettyWriter<T>,
        module_name: &str,
        name: &str,
        params_decomposed: &[ASTypeDecomposed],
        results_decomposed: &[ASTypeDecomposed],
        result: &ASResult,
    ) -> Result<(), Error> {
        let results_decomposed_deref = results_decomposed
            .iter()
            .map(|result_ptr_type| match result_ptr_type.type_.as_ref() {
                ASType::MutPtr(result_type) => ASTypeDecomposed {
                    name: result_ptr_type.name.clone(),
                    type_: result_type.clone(),
                },
                _ => panic!("Result type is not a pointer"),
            })
            .collect::<Vec<_>>();
        let results_set = results_decomposed_deref
            .iter()
            .map(|result| result.type_.as_lang())
            .collect::<Vec<_>>();
        let rust_fn_result_str = match results_set.len() {
            0 => "()".to_string(),
            1 => results_set[0].clone(),
            _ => format!("({})", results_set.join(", ")),
        };
        w.indent()?.write(format!(
            "Result<{}, Error> {}(",
            rust_fn_result_str,
            name.as_fn()
        ))?;
        if !params_decomposed.is_empty() || !results_decomposed.is_empty() {
            w.eol()?;
        }
        for param in params_decomposed {
            w.write_line_continued(format!(
                "{} {},",
                param.type_.as_lang(),
                param.name.as_var(),
            ))?;
        }
        w.write_line(format!(") {{"))?;
        {
            let mut w = w.new_block();

            // Inner (raw) definition
            {
                w.write_line(format!(
                    "final {} = DynamicLibrary.open('{}').lookupFunction<",
                    name.as_fn(),
                    module_name,
                ))?;
                {
                    let mut w = w.new_block();
                    w.write_line(format!("{} Function(", result.error_type.ffi_type()))?;
                    if !params_decomposed.is_empty() {
                        w.eol()?;
                    }
                    for param in params_decomposed.iter().chain(results_decomposed.iter()) {
                        w.write_line_continued(format!(
                            "{} {},",
                            param.type_.ffi_type(),
                            param.name.as_var(),
                        ))?;
                    }
                    w.indent()?.write("),")?;
                    let mut w = w.new_block();
                    w.write_line(format!("{} Function(", result.error_type.as_lang()))?;
                    if !params_decomposed.is_empty() {
                        w.eol()?;
                    }
                    for param in params_decomposed.iter().chain(results_decomposed.iter()) {
                        w.write_line_continued(format!(
                            "{} {},",
                            param.type_.as_lang(),
                            param.name.as_var(),
                        ))?;
                    }
                    w.indent()?.write(format!(")>('{}');", name))?;
                }
            }

            // Wrapper
            for result in &results_decomposed_deref {
                w.write_line(format!(
                    "final {} = malloc<{}>();",
                    result.name.as_var(),
                    result.type_.ffi_type(),
                ))?;
            }

            w.write_line(format!("final res = {}(", name.as_fn()))?;
            for param in params_decomposed {
                w.write_line_continued(format!("{},", param.name.as_var()))?;
            }
            for result in results_decomposed_deref.iter() {
                w.write_line_continued(format!("{},", result.name.as_var()))?;
            }
            w.write_line(");")?;
            w.write_lines(
                "if (res != 0) {
    return Err(WasiError(res));
}",
            )?;
            let res_str = match results_decomposed.len() {
                0 => "()".to_string(),
                1 => format!("{}.ref", results_decomposed_deref[0].name.as_var()),
                _ => format!(
                    "({})",
                    results_decomposed_deref
                        .iter()
                        .map(|result| format!("{}.ref", result.name.as_var()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            };
            w.write_line(format!("return Ok({});", res_str))?;
        };
        w.write_line("}")?;
        w.eob()?;

        Ok(())
    }
}
