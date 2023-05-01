use std::io::Write;

use super::*;

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

impl DartGenerator {
    pub fn define_as_tuple<T: Write>(
        w: &mut PrettyWriter<T>,
        name: &str,
        members: &[ASTupleMember],
    ) -> Result<(), Error> {
        w.write_line(format!(
            "final class {} extends Struct {{ // -- Tuple",
            name.as_type()
        ))?;
        {
            let mut w = w.new_block();
            for (i, member) in members.iter().enumerate() {
                let member_type = member.type_.as_ref();
                w.write_line(format!("{} v{};", member_type.field_definition(), i))?;

                write_padding(&mut w, member.padding)?;
            }
        }
        w.write_line("}")?.eob()?;
        Ok(())
    }
}
