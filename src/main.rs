use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read, Write, Error, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;
use serde::export::fmt::Debug;
use std::borrow::Borrow;

fn main() {
    run(TsScaffoldCommand::from_args()).unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "tsscaffold",
about = "Does useful stuff for you if you are writing a TS + Postgres application using PgTyped library."
)]
struct TsScaffoldCommand {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(subcommand)]
    command: TsScaffoldSubCommand,
}

struct IO<R: Read, W: Write> {
    input: R,
    output: W,
}

#[derive(Debug, StructOpt)]
enum TsScaffoldSubCommand {
    Insert {},
    CreateTable {},
}

fn run(opt: TsScaffoldCommand) -> io::Result<()> {
    let mut input: Box<dyn Read> = match opt.input {
        None => Box::new(io::stdin()),
        Some(i) => Box::new(fs::File::open(i)?)
    };

    let mut output: Box<dyn Write> = match opt.output {
        None => Box::new(io::stdout()),
        Some(o) => Box::new(fs::File::create(o)?)
    };

    let tables: Vec<Table> = parse_yaml(input)?;

    match opt.command {
        Insert => insert(tables, output),
        CreateTable => unimplemented!()
    };

    return Ok(());
}

fn parse_yaml<R: Read>(mut reader: R) -> io::Result<Vec<Table>> {
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
struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn get_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }

    fn get_pk_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter(|c| c.is_pk)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }

    fn get_non_pk_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter(|c| !c.is_pk)
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    sql_type: String,
    is_pk: bool,
}

fn yaml_to_tables(yaml: BTreeMap<String, Vec<String>>) -> Vec<Table> {
    let mut tables: Vec<Table> = vec![];
    for (table, column_strings) in yaml.iter() {
        let mut columns: Vec<Column> = vec![];
        for column_string in column_strings.iter() {
            columns.push(Column {
                name: column_string.clone(),
                sql_type: String::from("VRC"),
                is_pk: false,
            })
        }
        tables.push(Table {
            name: table.clone(),
            columns,
        })
    }
    tables
}

fn insert<W: Write>(tables: Vec<Table>, mut writer: W) -> io::Result<()> {
    for table in tables.iter() {
        writeln!(writer, "INSERT INTO {} (", table.name);
        let column_names = &table.get_column_names();
        writeln!(writer, "  {}", column_names.join(",\n  "));
        writeln!(writer, ") VALUES :rows");
        writeln!(
            writer,
            "ON CONFLICT ({}) DO UPDATE SET ",
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
