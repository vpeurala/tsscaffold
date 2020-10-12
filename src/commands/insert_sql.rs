use crate::domain::{Table, Column};
use heck::CamelCase;
use heck::MixedCase;
use std::io;
use std::io::Write;

pub fn insert_sql<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        let columns = &table.columns;
        let non_generated_column_names: Vec<String> = columns.iter().filter(|c| !c.is_generated).map(|c| c.name.clone()).collect();
        writeln!(writer, "/*")?;
        writeln!(writer, "@name Insert{}", table.name.to_camel_case())?;
        writeln!(
            writer,
            "@param rows -> (({})...)",
            non_generated_column_names
                .iter()
                .map(|s| s.to_mixed_case())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(writer, "*/")?;
        writeln!(writer, "INSERT INTO {} (", table.name)?;
        writeln!(writer, "  {}", non_generated_column_names.join(",\n  "))?;
        write!(writer, ") VALUES :rows")?;
        if table.get_pk_column_names().is_empty() {
            writeln!(writer, ";")?;
        } else {
            writeln!(writer, "")?;
            writeln!(
                writer,
                "ON CONFLICT ({}) DO UPDATE SET",
                table.get_pk_column_names().join(", ")
            )?;
            // ordinary_columns consists of columns which are not primary keys and are not generated.
            let ordinary_columns: Vec<&Column> = table.columns.iter().filter(|c| !c.is_generated && !c.is_pk).collect();
            let ordinary_columns_length = ordinary_columns.len();
            for (idx, non_pk_column) in ordinary_columns.iter().enumerate() {
                writeln!(
                    writer,
                    "  {} = EXCLUDED.{}{}",
                    non_pk_column.name,
                    non_pk_column.name,
                    if idx != (ordinary_columns_length - 1) {
                        ","
                    } else {
                        ";"
                    }
                )?;
            }
        }
    }
    Ok(())
}
