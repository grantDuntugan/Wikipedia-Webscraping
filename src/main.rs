extern crate select;
extern crate reqwest;

// HTML parsing
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

//User input
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ask user for wikipedia link
    println!("What do you want to search on wikipedia? (Bear, Family, Animal, etc.) ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Format link
    input = format_wikipedia(input);

    // Get html content using reqwest
    let resp = reqwest::blocking::get(input)?
        .text()?;
    
    // Find all "p" nodes from the html and print
    let document = Document::from(resp.as_str());
    for node in document.find(Name("p")) {
        println!("{}", node.text());
    }
    
    Ok(())
}

// Formats user input to be a wikipedia link
fn format_wikipedia(s: String) -> String {
    let mut result = String::new();
    result.push_str("https://en.wikipedia.org/wiki/");

    let tokens: Vec<&str> = s.split(" ").collect();
    for i in 0..tokens.len() {
        result.push_str(tokens[i]);
        if i == tokens.len() - 1 {
            break;
        }
        result.push('_');
    }

    return result;
}
