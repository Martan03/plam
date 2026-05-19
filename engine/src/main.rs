use std::{collections::HashMap, fs::File, io::BufReader, process::ExitCode};

use pareg::Pareg;
use termal::eprintacln;
use utf8_chars::BufReadCharsExt;

use crate::{
    cli::Args, err::Result, expr::ExprTree, i_tab::ITab,
    interpreter::init_interpreter, parser::parse,
};

mod cli;
mod err;
mod expr;
mod i_tab;
mod interpreter;
mod lam_repr;
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
    let mut et = ExprTree::new();
    let mut exprs = vec![];
    // Define builtins
    defs.insert(itab.insert("$increment"), et.increment());
    defs.insert(itab.insert("$counter"), et.counter(0));
    defs.insert(itab.insert("$char"), et.char());
    defs.insert(itab.insert("$stdin"), et.stdin(0));

    // Load the code
    for p in args.sources {
        let mut file = BufReader::new(File::open(&p)?);
        let chars = file.chars().map(|e| e.map_err(|e| e.into()));
        let e = parse(&mut itab, &mut et, chars, &mut defs)
            .map_err(|e| e.with_path(p))?;
        exprs.extend(e);
    }

    // Interpret the code.
    let mut int = init_interpreter(&mut et, defs, &mut itab);
    int.cache_limit = args.cache_limit;
    let mut buf = String::new();
    for expr in exprs {
        int.eval(&expr);
        buf.clear();
        int.et.to_string(&expr, int.itab, &mut buf);
        println!("{buf}");
    }

    Ok(())
}
