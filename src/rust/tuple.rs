use super::*;
use std::io::Write;

pub struct Tuple;

impl Tuple {
    pub fn name_for(tuple_members: &[ASTupleMember]) -> String {
        format!(
            "WasiTuple{}{}",
            tuple_members.len(),
            tuple_members
                .iter()
                .map(|member| member.type_.to_string())
                .collect::<Vec<_>>()
                .join("_")
        )
    }
}

impl RustGenerator {
    pub fn define_as_tuple<T: Write>(
        w: &mut PrettyWriter<T>,
        name: &str,
        members: &[ASTupleMember],
    ) -> Result<(), Error> {
        w.write_line("#[repr(C, packed)]")?
            .write_line("#[derive(Copy, Clone, Debug)]")?
            .write_line(format!("pub struct {} {{ // -- Tuple", name.as_type()))?;
        {
            let mut w = w.new_block();
            for (i, member) in members.iter().enumerate() {
                let member_type = member.type_.as_ref();
                w.write_line(format!("pub v{}: {},", i, member_type.as_lang()))?;

                let pad_len = member.padding;
                for i in 0..(pad_len & 1) {
                    w.write_line(format!("__pad8_{}: u8,", i))?;
                }
                for i in 0..(pad_len & 3) / 2 {
                    w.write_line(format!("__pad16_{}: u16,", i))?;
                }
                for i in 0..(pad_len & 7) / 4 {
                    w.write_line(format!("__pad32_{}: u32,", i))?;
                }
                for i in 0..pad_len / 8 {
                    w.write_line(format!("__pad64_{}: u64,", i))?;
                }
            }
        }
        w.write_line("}")?.eob()?;
        Ok(())
    }
}
