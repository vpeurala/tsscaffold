use crate::domain::{Column, Table};
use std::io;
use std::io::Write;

pub fn create_table<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        writeln!(writer, "CREATE TABLE {} (", table.name)?;
        let columns = &table.columns;
        for (idx, col) in columns.iter().enumerate() {
            write!(
                writer,
                "    {} {} {}",
                col.name,
                col.sql_type,
                if col.is_nullable { "NULL" } else { "NOT NULL" }
            )?;
            if idx != columns.len() - 1 {
                writeln!(writer, ",")?;
            } else {
                writeln!(writer, "\n);")?;
            }
        }
        if columns.iter().any(|c| c.is_pk) {
            let pk_columns = columns.iter().filter(|c| c.is_pk).collect::<Vec<&Column>>();
            writeln!(writer)?;
            writeln!(
                writer,
                "ALTER TABLE {} ADD CONSTRAINT {} PRIMARY KEY ({});",
                &table.name,
                table.name.clone() + &String::from("_pk"),
                pk_columns
                    .iter()
                    .map(|c| c.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
    }
    Ok(())
}
