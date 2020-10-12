extern crate heck;
#[macro_use]
extern crate maplit;

pub mod domain;

pub mod commands;

use std::collections::{BTreeMap, HashSet};
use std::io;
use std::io::{Error, ErrorKind, Read};

use domain::{Column, Table};

pub fn parse_yaml<R: Read>(mut reader: R) -> io::Result<Vec<Table>> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let yaml_parse_result: Result<BTreeMap<String, Vec<String>>, serde_yaml::Error> =
        serde_yaml::from_str(&buffer);
    return match yaml_parse_result {
        Ok(yaml) => Ok(yaml_to_tables(yaml)),
        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };
}

pub fn yaml_to_tables(yaml: BTreeMap<String, Vec<String>>) -> Vec<Table> {
    let mut tables: Vec<Table> = vec![];
    for (table, column_strings) in yaml.iter() {
        let mut columns: Vec<Column> = vec![];
        for column_string in column_strings.iter() {
            let parts = column_string.split_whitespace();
            let mut column_name = "";
            let mut column_properties: HashSet<String> = HashSet::new();
            for (index, part) in parts.enumerate() {
                if index == 0 {
                    column_name = part;
                } else {
                    column_properties.insert(part.to_uppercase());
                }
            }
            let keywords = hashset! {String::from("PK"), String::from("NULLABLE")};
            columns.push(Column {
                name: column_name.to_owned(),
                is_pk: column_properties.contains("PK"),
                is_nullable: column_properties.contains("NULLABLE"),
                sql_type: column_properties
                    .difference(&keywords)
                    .next()
                    .unwrap_or(&String::from("TEXT"))
                    .clone(),
            })
        }
        tables.push(Table {
            name: table.clone(),
            columns,
        })
    }
    tables
}
