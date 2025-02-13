pub fn example_function(input: String) -> String {
    format!("You passed: {}", input)
}

fn main() {
    let result = example_function("Hello".to_string());
    println!("{}", result); // Output: You passed: Hello
}
