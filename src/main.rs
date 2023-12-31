use chatgpt::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use chatgpt::types::{CompletionResponse};
// use std::env;
use clap::Parser;

/// Simple CLI for OpenAI chat-gpt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Prompt to ask chat-gpt
    #[arg(short, long)]
    prompt: String,

    /// OpenAI API key to use
    // #[arg(short, long)]
    // key: String,

    /// Path to config.yml for storing API key and other options
    #[arg(short, long)]
    config: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    openai_api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    let f = std::fs::File::open(args.config).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    //println!("{:?}", scrape_config);

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let client = ChatGPT::new(config.openai_api_key)?;

    // Sending a message and getting the completion
    // println!("Prompt: {}", args[1..args.len()].join(" "));
    // args[1..args.len()].join(" ")
    let response: CompletionResponse = client
       .send_message(args.prompt)
       .await?;

    println!("{}", response.message().content);

    Ok(())
}
