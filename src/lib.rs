#[macro_use]
extern crate maplit;

use std::collections::{BTreeMap, HashSet};
use std::io;
use std::io::{Error, ErrorKind, Read, Write};

use heck::{CamelCase, MixedCase};

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

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn get_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|_c: &&Column| true)
    }

    pub fn get_pk_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|c: &&Column| c.is_pk)
    }

    pub fn get_non_pk_column_names(&self) -> Vec<String> {
        self.filter_columns_and_extract_names(|c: &&Column| !c.is_pk)
    }

    fn filter_columns_and_extract_names(
        &self,
        predicate: impl FnMut(&&Column) -> bool,
    ) -> Vec<String> {
        self.columns
            .iter()
            .filter(predicate)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    sql_type: String,
    is_pk: bool,
    is_nullable: bool,
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
                    .unwrap()
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
