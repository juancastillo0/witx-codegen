use std::io::Write;

use super::*;

impl DartGenerator {
    pub fn define_as_struct<T: Write>(
        w: &mut PrettyWriter<T>,
        name: &str,
        members: &[ASStructMember],
    ) -> Result<(), Error> {
        w.write_line(format!("final class {} extends Struct {{", name.as_type()))?;
        {
            let mut w = w.new_block();
            for member in members {
                let member_type = member.type_.as_ref();
                w.write_line(format!(
                    "{} {};",
                    member_type.field_definition(),
                    member.name.as_var()
                ))?;

                write_padding(&mut w, member.padding)?;
            }
        }
        w.write_line("}")?.eob()?;
        Ok(())
    }
}
