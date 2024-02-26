use affine_lambda_calc::normalize;
use affine_lambda_calc::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<char> = "(@x.x y)".chars().collect();
    let mut parser = Parser::new(input);
    let res = parser.parse();
    if let Some(term) = res {
        term.print();
        let normalized = normalize(term);
        println!("Normalized term: ");
        normalized.print();
    } else {
        println!("Failed to parse input.");
    }
    Ok(())
}
