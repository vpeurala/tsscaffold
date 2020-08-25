use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    println!("Hello, kuukkers!");
    run(TsScaffoldCommand::from_args()).unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "tsscaffold",
about = "Does useful stuff for you if you are writing a TS + Postgres application using PgTyped library."
)]
enum TsScaffoldCommand {
    Insert {
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        output: Option<PathBuf>,
    },
}

fn run(opt: TsScaffoldCommand) -> io::Result<()> {
    match opt {
        TsScaffoldCommand::Insert { input, output } => match (input, output) {
            (None, None) => parse_yaml(io::stdin(), io::stdout())?,
            (Some(i), Some(o)) => parse_yaml(fs::File::open(i)?, fs::File::create(o)?)?,
            (None, Some(o)) => parse_yaml(io::stdin(), fs::File::create(o)?)?,
            (Some(i), None) => parse_yaml(fs::File::open(i)?, io::stdout())?,
        },
    }
    Ok(())
}

fn parse_yaml<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let yaml_parse_result: Result<BTreeMap<String, Vec<String>>, serde_yaml::Error> =
        serde_yaml::from_str(&buffer);
    return match yaml_parse_result {
        Ok(yaml) => insert(yaml_to_tables(yaml), writer),
        Err(err) => write!(writer, "Invalid YAML:\n{:?}\n", err)
    };
}

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn get_column_names(&self) -> Vec<String> {
        self.columns.iter().map(|c| c.name.clone()).collect::<Vec<String>>()
    }

    fn get_pk_column_names(&self) -> Vec<String> {
        self.columns.iter().filter(|c| c.is_pk).map(|c| c.name.clone()).collect::<Vec<String>>()
    }

    fn get_non_pk_column_names(&self) -> Vec<String> {
        self.columns.iter().filter(|c| !c.is_pk).map(|c| c.name.clone()).collect::<Vec<String>>()
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
        writeln!(writer, "ON CONFLICT ({}) DO UPDATE SET ", table.get_pk_column_names().join(", "));
        for non_pk_column_name in table.get_non_pk_column_names().iter() {
            writeln!(writer, "  {} = EXCLUDED.{}", non_pk_column_name, non_pk_column_name);
        }
        writeln!(writer, ";");
    }
    Ok(())
}
