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
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        input: Option<PathBuf>,
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        output: Option<PathBuf>,
    },
}

fn run(opt: TsScaffoldCommand) -> io::Result<()> {
    if let TsScaffoldCommand::Insert { input, output } = opt {
        match (input, output) {
            (None, None) => insert(io::stdin(), io::stdout())?,
            (Some(i), Some(o)) => insert(fs::File::open(i)?, fs::File::create(o)?)?,
            (None, Some(o)) => insert(io::stdin(), fs::File::create(o)?)?,
            (Some(i), None) => insert(fs::File::open(i)?, io::stdout())?,
        }
    }
    Ok(())
}

fn insert<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    writer.write(buffer.as_bytes());

    

    Ok(())
}
