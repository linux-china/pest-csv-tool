extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::{state, ParseResult, Parser, ParserState};

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    match CSVParser::parse(Rule::file, "111, 123, 1234.11, 1234\n") {
        Ok(pairs) => {
            for value in fields(pairs) {
                println!("{}", value);
            }
        }
        Err(e) => println!("\n{}", e),
    };
}

fn fields(pairs: Pairs<Rule>) -> Vec<&str> {
    pairs
        .filter(|p| p.as_rule() == Rule::file)
        .flat_map(|p| p.into_inner().filter(|p2| p2.as_rule() == Rule::record))
        .flat_map(|p| p.into_inner().filter(|p2| p2.as_rule() == Rule::field))
        .map(|p| p.as_span().as_str())
        .collect()
}

