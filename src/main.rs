extern crate heck;

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use serde::export::fmt::Debug;
use structopt::StructOpt;

use tsscaffold::{insert, parse_yaml, Table};

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

#[derive(Debug, StructOpt)]
enum TsScaffoldSubCommand {
    Insert {},
    CreateTable {},
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

    match opt.command {
        TsScaffoldSubCommand::Insert {} => insert(tables, output)?,
        TsScaffoldSubCommand::CreateTable {} => unimplemented!(),
    };

    return Ok(());
}
