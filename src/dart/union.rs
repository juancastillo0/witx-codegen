use std::io::Write;

use super::*;

impl DartGenerator {
    fn define_union_member_accessors<T: Write>(
        w: &mut PrettyWriter<T>,
        union_name: &str,
        i: usize,
        member: &ASUnionMember,
        inner_name: &str,
    ) -> Result<(), Error> {
        let name = &member.name;
        let member_is_void = matches!(member.type_.as_ref(), ASType::Void);

        if member_is_void {
            // new_*
            w.write_line(format!(
                "{} new{}() {{",
                union_name.as_type(),
                name.as_fn_suffix()
            ))?;
            {
                let mut w = w.new_block();
                w.write_line(format!("return fromTag({});", i))?;
            }
            w.write_line("}")?.eob()?;
        } else {
            // !member_is_void
            // new_*
            w.write_line(format!(
                "{} new{}({} val) {{",
                union_name.as_type(),
                name.as_fn_suffix(),
                member.type_.as_lang()
            ))?;
            {
                let mut w = w.new_block();
                w.write_line(format!("final tu = fromTag({});", i))?;
                w.write_line(format!("tu.member.ref.{} = val;", member.name.as_var()))?;
                w.write_line("return tu;")?;
            }
            w.write_line("}")?.eob()?;

            // get_*
            w.write_line(format!(
                "{} into{}() {{",
                member.type_.as_lang(),
                name.as_fn_suffix(),
            ))?;
            {
                let mut w = w.new_block();
                w.write_line(format!("assert(tag == {});", i))?;
                w.write_line(format!("return member.ref.{};", member.name.as_var()))?;
            }
            w.write_line("}")?.eob()?;

            // set_*
            w.write_line(format!(
                "void set{}({} val) {{",
                name.as_fn_suffix(),
                member.type_.as_lang()
            ))?;
            {
                let mut w = w.new_block();
                w.write_line(format!("assert(tag == {});", i))?;
                w.write_line(format!("member.ref.{} = val;", member.name.as_var()))?;
            }
            w.write_line("}")?.eob()?;
        }

        // is_*
        w.write_line(format!("bool is{}() {{", name.as_fn_suffix()))?;
        {
            let mut w = w.new_block();
            w.write_line(format!("return tag == {};", i))?;
        }
        w.write_line("}")?.eob()?;

        Ok(())
    }

    fn define_union_member<T: Write>(
        w: &mut PrettyWriter<T>,
        union_name: &str,
        i: usize,
        member: &ASUnionMember,
        inner_name: &str,
    ) -> Result<(), Error> {
        let member_type = member.type_.as_ref();
        match member_type {
            ASType::Void => {
                w.write_line(format!(
                    "// --- {}: (no associated content) if tag={}",
                    member.name.as_var(),
                    i
                ))?;
            }
            _ => {
                w.write_line(format!(
                    "// --- {}: {} if tag={}",
                    member.name.as_var(),
                    member_type.as_lang(),
                    i
                ))?;
            }
        }
        w.eob()?;
        Self::define_union_member_accessors(w, union_name, i, member, inner_name)?;
        Ok(())
    }

    pub fn define_as_union<T: Write>(
        w: &mut PrettyWriter<T>,
        name: &str,
        union_: &ASUnion,
    ) -> Result<(), Error> {
        let tag_repr = union_.tag_repr.as_ref();
        let inner_name = format!("{}_member", name);
        w.write_line(format!(
            "final class {} extends Union {{",
            inner_name.as_type()
        ))?;
        {
            let mut w = w.new_block();
            for (i, member) in union_.members.iter().enumerate() {
                let member_is_void = matches!(member.type_.as_ref(), ASType::Void);
                if member_is_void {
                    w.write_line(format!(
                        "// {} with no associated value if tag={}",
                        member.name.as_var(),
                        i
                    ))?;
                } else {
                    w.write_line(format!(
                        "{} {}; // if tag={}",
                        member.type_.field_definition(),
                        member.name.as_var(),
                        i
                    ))?;
                }
            }
        }
        w.write_line("}")?;
        w.eob()?;

        w.write_line(format!("final class {} extends Struct {{", name.as_type()))?;
        {
            let mut w = w.new_block();
            w.write_line(format!("{} tag;", tag_repr.field_definition()))?;

            write_padding(&mut w, union_.padding_after_tag)?;
            w.write_line(format!(
                "external Pointer<{}> member;",
                inner_name.as_type()
            ))?;
        }

        {
            let mut w = w.new_block();
            w.write_line(format!(
                "static {} fromTag({} tag) {{",
                name.as_type(),
                tag_repr.as_lang()
            ))?;
            {
                let mut w = w.new_block();
                w.write_line(format!("final tu = malloc<{}>();", name.as_type()))?;
                w.write_line("tu.ref.tag = tag;")?;
                w.write_line("return tu.ref;")?;
            }
            w.write_line("}")?.eob()?;

            for (i, member) in union_.members.iter().enumerate() {
                w.eob()?;
                Self::define_union_member(&mut w, name, i, member, &inner_name)?;
            }
        }
        w.write_line("}")?.eob()?;
        Ok(())
    }
}
