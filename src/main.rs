mod parser;

use parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<char> = "(@x.x @y.y)".chars().collect();
    let mut parser = Parser::new(input);
    let res = parser.parse();
    if let Some(term) = res {
        term.print();
    } else {
        println!("Failed to parse input.");
    }
    Ok(())
}
