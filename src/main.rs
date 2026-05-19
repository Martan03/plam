use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, StdinLock},
    process::ExitCode,
};

use pareg::Pareg;
use termal::eprintacln;
use utf8_chars::BufReadCharsExt;

use crate::{
    cli::Args,
    err::Result,
    expr::{ExprId, ExprTree},
    i_tab::{ITab, Id},
    interpreter::Interpreter,
    lam_repr::{
        Bottom, First, Incr, List, PeanoChars, Second, StdinList, Triple,
        YComb,
    },
    parser::parse,
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
    let mut buf = String::new();
    for expr in exprs {
        int.eval(&expr, args.expand);
        buf.clear();
        int.et.to_string(&expr, &itab, &mut buf);
        println!("{buf}");
    }

    Ok(())
}

fn init_interpreter<'a>(
    et: &'a mut ExprTree,
    defs: HashMap<Id, ExprId>,
    itab: &mut ITab,
) -> Interpreter<'a, StdinLock<'static>> {
    let y = YComb::new(et, itab);
    let first = First::new(et, itab);
    let triple = Triple::new(et, itab);
    let second = Second::new(et, itab);
    let incr = Incr::new(et, itab);
    let bottom = Bottom::new(et, y, first.clone());
    let list =
        List::new(et, triple, first, second.clone(), bottom.clone(), itab);
    let pean_chars = PeanoChars::new(et, &incr, second);
    let stdin = std::io::stdin().lock();
    let stdin_list = StdinList::new(stdin, pean_chars, list, bottom);
    Interpreter::new(et, defs, stdin_list)
}
