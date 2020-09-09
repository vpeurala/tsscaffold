use crate::domain::Table;
use heck::CamelCase;
use heck::MixedCase;
use std::io;
use std::io::Write;

pub fn insert_interface<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        writeln!(
            writer,
            "export interface IInsert{}Row {{",
            table.name.to_camel_case()
        )?;
        for (idx, column) in table.columns.iter().enumerate() {
            writeln!(
                writer,
                "    {}: {}{}{}",
                column.name.to_mixed_case(),
                ts_type(&column.sql_type),
                if column.is_nullable { " | null" } else { "" },
                if idx != (table.columns.len() - 1) {
                    ","
                } else {
                    ""
                }
            )?;
        }
        writeln!(writer, "}}")?;
    }
    Ok(())
}

fn ts_type(sql_type: &String) -> String {
    match &sql_type.clone().to_uppercase()[..] {
        "VARCHAR" => String::from("string"),
        "INTEGER" => String::from("number"),
        "DECIMAL" => String::from("number"),
        "DATE" => String::from("Date"),
        otherwise => panic!("Unknown SQL type: {}", otherwise),
    }
}
