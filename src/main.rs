use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use serde::export::fmt::Debug;
use structopt::StructOpt;

use tsscaffold::commands::create_table;
use tsscaffold::commands::insert_sql;
use tsscaffold::commands::insert_interface;
use tsscaffold::domain::Table;
use tsscaffold::{parse_yaml};

fn main() -> io::Result<()> {
    run(TsScaffoldCommand::from_args())
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
    kind: TsScaffoldSubCommandKind
}

#[derive(Debug, StructOpt)]
enum TsScaffoldSubCommandKind {
    InsertSql,
    CreateTable,
    InsertInterface
}

impl TsScaffoldSubCommandKind {
    fn run<W: Write>(&self, tables: Vec<Table>, output: W) -> io::Result<()> {
        match self {
            TsScaffoldSubCommandKind::InsertSql => insert_sql(tables, output),
            TsScaffoldSubCommandKind::CreateTable => create_table(tables, output),
            TsScaffoldSubCommandKind::InsertInterface => insert_interface(tables, output)
        }
    }
}

fn run(opt: TsScaffoldCommand) -> io::Result<()> {
    let input: Box<dyn Read> = match opt.input {
        None => Box::new(io::stdin()),
        Some(i) => Box::new(fs::File::open(i)?),
    };

    let output: Box<dyn Write> = match opt.output {
        None => Box::new(io::stdout()),
        Some(o) => Box::new(fs::File::create(o)?),
    };

    let tables: Vec<Table> = parse_yaml(input)?;

    opt.kind.run(tables, output)
}
