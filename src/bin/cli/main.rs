use std::env;

use url2ref::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let reference = generate(query, url2ref::GenerationOptions::default()).unwrap();
    println!("{}", reference.wiki())
}