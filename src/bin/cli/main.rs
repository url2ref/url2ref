use clap::Parser;

use url2ref::generate_reference;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandLineArgs {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    ref_type: Option<String>,
}


fn main() {
    let args = CommandLineArgs::parse();
    let query = args.url;

    let reference_result = generate_reference(&query);
    let reference = match reference_result {
        Ok(res) => res,
        Err(error) => panic!("Reference generation failed: {}", error),
    };
    println!("{}", reference)
}