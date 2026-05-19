use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    err::Result, expr::ExprTree, i_tab::ITab, interpreter::init_interpreter,
    parser::parse,
};

mod err;
mod expr;
mod i_tab;
mod interpreter;
mod lam_repr;
mod parser;

#[wasm_bindgen]
pub fn eval_lambda(input: &str) -> Result<String> {
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

    let chars = input.chars().map(|e| Ok(e));
    let e = parse(&mut itab, &mut et, chars, &mut defs)?;
    exprs.extend(e);

    // Interpret the code.
    let mut int = init_interpreter(&mut et, defs, &mut itab);
    let mut buf = String::new();

    let mut res = String::new();
    for expr in exprs {
        int.eval(&expr);
        buf.clear();
        buf.clear();
        int.et.to_string(&expr, int.itab, &mut buf);

        res.push_str(&buf);
        res.push('\n');
    }

    Ok(res)
}
