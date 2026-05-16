use std::{
    collections::HashMap, env, fs::File, io::BufReader, process::ExitCode,
    rc::Rc,
};

use termal::eprintacln;
use utf8_chars::BufReadCharsExt;

use crate::{
    err::Result, expr::Expr, i_tab::ITab, interpreter::Interpreter,
    parser::parse,
};

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
    let args: Vec<_> = env::args().collect();
    let mut file = BufReader::new(File::open(&args[1])?);
    let chars = file.chars().map(|e| e.map_err(|e| e.into()));

    // Prepare data containers
    let mut itab = ITab::new();
    let mut defs = HashMap::new();
    // Define builtins
    defs.insert(itab.insert("$increment"), Rc::new(Expr::Increment));
    defs.insert(itab.insert("$counter"), Rc::new(Expr::Counter(0)));

    // Parse the code.
    let exprs = parse(&mut itab, chars, &mut defs)?;

    // Interpret the code.
    let int = Interpreter::new(defs);
    let mut buf = String::new();
    for expr in exprs {
        let val = int.eval(expr);
        buf.clear();
        val.to_string(&itab, &mut buf);
        println!("{buf}");
    }

    Ok(())
}
