use crate::domain::Table;
use heck::CamelCase;
use heck::MixedCase;
use std::io;
use std::io::Write;

pub fn insert<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        let column_names = &table.get_column_names();
        writeln!(writer, "/*")?;
        writeln!(writer, "@name Insert{}", table.name.to_camel_case())?;
        writeln!(
            writer,
            "@param rows -> (({})...)",
            column_names
                .iter()
                .map(|s| s.to_mixed_case())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(writer, "*/")?;
        writeln!(writer, "INSERT INTO {} (", table.name)?;
        writeln!(writer, "  {}", column_names.join(",\n  "))?;
        writeln!(writer, ") VALUES :rows")?;
        writeln!(
            writer,
            "ON CONFLICT ({}) DO UPDATE SET",
            table.get_pk_column_names().join(", ")
        )?;
        for (idx, non_pk_column_name) in table.get_non_pk_column_names().iter().enumerate() {
            writeln!(
                writer,
                "  {} = EXCLUDED.{}{}",
                non_pk_column_name,
                non_pk_column_name,
                if idx != (table.get_non_pk_column_names().len() - 1) {
                    ","
                } else {
                    ";"
                }
            )?;
        }
    }
    Ok(())
}
