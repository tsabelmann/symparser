extern crate pest;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar/sym.pest"]
pub struct SymParser;
