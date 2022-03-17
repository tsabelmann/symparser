use pest::Parser;
use std::fs::read_to_string;
use symparser::v6::{Rule, SymParser};

#[test]
fn parse_001() {
    let string = read_to_string("examples/example_004.sym").unwrap();
    let _pairs = SymParser::parse(Rule::main, &string).unwrap();
}

#[test]
fn parse_002() {
    let string = read_to_string("examples/example_004.sym").unwrap();
    let mut pairs = SymParser::parse(Rule::main, &string).unwrap().into_iter();
    assert_eq!(pairs.next().is_some(), true);
    assert_eq!(pairs.next().is_some(), false);
}

#[test]
fn parse_003() {
    let string = read_to_string("examples/example_004.sym").unwrap();
    let mut pairs = SymParser::parse(Rule::sym, &string).unwrap().into_iter();
    let mut inner_pair = pairs.next().unwrap().into_inner();

    assert_eq!(
        inner_pair.next().unwrap().as_str().trim(),
        "FormatVersion=6.0"
    );
    assert_eq!(
        inner_pair.next().unwrap().as_str().trim(),
        "Title=\"EXAMPLE\""
    );
    assert_eq!(inner_pair.next().unwrap().as_str().trim(), "{ENUMS}");
    assert_eq!(inner_pair.next().unwrap().as_str().trim(), "{SIGNALS}");
    assert_eq!(inner_pair.next().unwrap().as_str().trim(), "{SEND}");
    assert_eq!(inner_pair.next().unwrap().as_str().trim(), "{RECEIVE}");
    assert_eq!(inner_pair.next().unwrap().as_str().trim(), "{SENDRECEIVE}");
    assert_eq!(inner_pair.next(), None);
}

#[test]
fn parse_004() {
    let string = read_to_string("examples/example_004.sym").unwrap();
    let mut pairs = SymParser::parse(Rule::sym, &string).unwrap().into_iter();
    let mut inner_pair = pairs.next().unwrap().into_inner();

    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::format_version);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::title);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::enums);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::signals);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::send);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::receive);
    assert_eq!(inner_pair.next().unwrap().as_rule(), Rule::sendreceive);
    assert_eq!(inner_pair.next(), None);
}
