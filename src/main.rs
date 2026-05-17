use std::{
    collections::HashMap, fs::File, io::BufReader, process::ExitCode, rc::Rc,
};

use pareg::Pareg;
use termal::eprintacln;
use utf8_chars::BufReadCharsExt;

use crate::{
    cli::Args, err::Result, expr::Expr, i_tab::ITab, interpreter::Interpreter,
    parser::parse,
};

mod cli;
mod err;
mod expr;
mod i_tab;
mod interpreter;
mod parser;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintacln!("{'r}error: {'_}{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    // Open the file.
    let args = Args::parse(Pareg::args())?;

    if args.sources.is_empty() {
        return Ok(());
    }

    // Prepare data containers
    let mut itab = ITab::new();
    let mut defs = HashMap::new();
    let mut exprs = vec![];
    // Define builtins
    defs.insert(itab.insert("$increment"), Rc::new(Expr::Increment));
    defs.insert(itab.insert("$counter"), Rc::new(Expr::Counter(0)));
    defs.insert(itab.insert("$char"), Rc::new(Expr::Char));

    // Load the code
    for p in args.sources {
        let mut file = BufReader::new(File::open(p)?);
        let chars = file.chars().map(|e| e.map_err(|e| e.into()));
        exprs.append(&mut parse(&mut itab, chars, &mut defs)?);
    }

    // Interpret the code.
    let int = Interpreter::new(defs);
    let mut buf = String::new();
    for expr in exprs {
        let val = int.eval(expr, args.expand);
        buf.clear();
        val.to_string(&itab, &mut buf);
        println!("{buf}");
    }

    Ok(())
}
