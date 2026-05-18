use std::{collections::HashMap, rc::Rc};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    err::Result, expr::Expr, i_tab::ITab, interpreter::Interpreter,
    parser::parse,
};

mod err;
mod expr;
mod i_tab;
mod interpreter;
mod parser;

#[wasm_bindgen]
pub fn eval_lambda(input: &str) -> Result<String> {
    // Prepare data containers
    let mut itab = ITab::new();
    let mut defs = HashMap::new();
    let mut exprs = vec![];
    // Define builtins
    defs.insert(itab.insert("$increment"), Rc::new(Expr::Increment));
    defs.insert(itab.insert("$counter"), Rc::new(Expr::Counter(0)));
    defs.insert(itab.insert("$char"), Rc::new(Expr::Char));

    let chars = input.chars().map(|e| Ok(e));
    let e = parse(&mut itab, chars, &mut defs)?;
    exprs.extend(e);

    // Interpret the code.
    let int = Interpreter::new(defs);
    let mut buf = String::new();

    let mut res = String::new();
    for expr in exprs {
        let val = int.eval(expr, false);
        buf.clear();
        val.to_string(&itab, &mut buf);
        res.push_str(&buf);
        res.push('\n');
    }

    Ok(res)
}
