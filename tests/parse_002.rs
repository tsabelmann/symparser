use pest::Parser;
use std::fs::read_to_string;
use symparser::v6::{Rule, SymParser};

#[test]
fn parse_001() {
    let string = read_to_string("examples/example_002.sym").unwrap();
    let _pairs = SymParser::parse(Rule::sym, &string).unwrap();
}

#[test]
fn parse_002() {
    let string = read_to_string("examples/example_002.sym").unwrap();
    let _pairs = SymParser::parse(Rule::sym, &string).unwrap();
}
