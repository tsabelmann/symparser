use pest::Parser;
use std::fs::read_to_string;
use symparser::v6::{Rule, SymParser};

#[test]
fn parse_001() {
    let string = read_to_string("examples/example_005.sym").unwrap();
    let _pairs = SymParser::parse(Rule::main, &string).unwrap();
}
