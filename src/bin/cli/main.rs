use clap::Parser;

use url2ref::*;

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

    let reference = generate(&query, url2ref::GenerationOptions::default()).unwrap();
    println!("{}", reference.wiki())
}