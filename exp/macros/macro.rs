#[proc_macro_attribute]
pub fn example_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Pass through the input code unchanged
    input
}

fn main() {
    let result = example_function("Hello".to_string());
    println!("{}", result); // Output: You passed: Hello
}
