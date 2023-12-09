use std::env;

use url2ref::generate_reference;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let reference_result = generate_reference(query);
    let reference = match reference_result {
        Ok(res) => res,
        Err(error) => panic!("Reference generation failed: {}", error),
    };
    println!("{}", reference)
}