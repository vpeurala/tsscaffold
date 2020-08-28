use std::collections::BTreeMap;
use std::io::{Error, ErrorKind, Read, Write};
use std::io;

use heck::{CamelCase, MixedCase};

pub fn insert<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        let column_names = &table.get_column_names();
        writeln!(writer, "/*");
        writeln!(writer, "@name Insert{}", table.name.to_camel_case());
        writeln!(writer, "@param rows -> (({})...)", column_names.iter().map(|s| s.to_mixed_case()).collect::<Vec<String>>().join(", "));
        writeln!(writer, "*/");
        writeln!(writer, "INSERT INTO {} (", table.name);
        writeln!(writer, "  {}", column_names.join(",\n  "));
        writeln!(writer, ") VALUES :rows");
        writeln!(
            writer,
            "ON CONFLICT ({}) DO UPDATE SET",
            table.get_pk_column_names().join(", ")
        );
        for non_pk_column_name in table.get_non_pk_column_names().iter() {
            writeln!(
                writer,
                "  {} = EXCLUDED.{}",
                non_pk_column_name, non_pk_column_name
            );
        }
        writeln!(writer, ";");
    }
    Ok(())
}

pub fn parse_yaml<R: Read>(mut reader: R) -> io::Result<Vec<Table>> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let yaml_parse_result: Result<BTreeMap<String, Vec<String>>, serde_yaml::Error> =
        serde_yaml::from_str(&buffer);
    return match yaml_parse_result {
        Ok(yaml) => Ok(yaml_to_tables(yaml)),
        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err.to_string()))
    };
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn get_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn get_pk_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter(|c| c.is_pk)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn get_non_pk_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter(|c| !c.is_pk)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    sql_type: String,
    is_pk: bool,
}

pub fn yaml_to_tables(yaml: BTreeMap<String, Vec<String>>) -> Vec<Table> {
    let mut tables: Vec<Table> = vec![];
    for (table, column_strings) in yaml.iter() {
        let mut columns: Vec<Column> = vec![];
        for column_string in column_strings.iter() {
            let parts = column_string.split_whitespace();
            let mut column_name = "";
            let mut column_sql_type = "";
            let mut column_is_part_of_primary_key = false;
            for (index, part) in parts.enumerate() {
                if index == 0 {
                    column_name = part;
                } else {
                    if part.eq("PK") {
                        column_is_part_of_primary_key = true;
                    } else {
                        column_sql_type = part;
                    }
                }
            }
            columns.push(Column {
                name: column_name.to_owned(),
                sql_type: column_sql_type.to_owned(),
                is_pk: column_is_part_of_primary_key,
            })
        }
        tables.push(Table {
            name: table.clone(),
            columns,
        })
    }
    tables
}

