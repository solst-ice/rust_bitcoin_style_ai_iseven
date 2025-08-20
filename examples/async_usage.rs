use rust_bitcoin_style_ai_iseven::is_even_async;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("Please set the ANTHROPIC_API_KEY environment variable");

    let test_numbers = vec![2, 3, 42, 69, 420, 1337];

    println!("Testing Rust Bitcoin-Style AI is-even (async):");
    println!("{}", "=".repeat(50));

    for number in test_numbers {
        match is_even_async(api_key.clone(), number).await {
            Ok(hash) => {
                println!("is_even({:4}) -> {}", number, hash);
            }
            Err(e) => {
                eprintln!("Error checking {}: {}", number, e);
            }
        }
    }

    println!("\nReference hashes:");
    println!("SHA256('True')  = 3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18");
    println!("SHA256('False') = 60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe");

    Ok(())
}