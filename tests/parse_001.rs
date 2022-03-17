use pest::Parser;
use std::fs::read_to_string;
use symparser::v6::{Rule, SymParser};

#[test]
fn parse_001() {
    let string = read_to_string("examples/example_001.sym").unwrap();
    let _pairs = SymParser::parse(Rule::main, &string).unwrap();
}

// #[test]
// fn parse_002() {
//     let string = read_to_string("examples/example_001.sym").unwrap();
//     let pairs = SymParser::parse(Rule::sym, &string).unwrap();
//     for pair in pairs {
//         println!("Rule:    {:?}", pair.as_rule());
//         // println!("Span:    {:?}", pair.as_span());
//         // println!("Text:    {}", pair.as_str());
//
//         // A pair can be converted to an iterator of the tokens which make it up:
//         for inner_pair in pair.into_inner() {
//             match inner_pair.as_rule() {
//                 Rule::format_version => println!("FormatVersion: {}", inner_pair.as_str()),
//                 Rule::title => println!("Title: {}", inner_pair.as_str()),
//                 _ => continue,
//             };
//         }
//     }
// }
