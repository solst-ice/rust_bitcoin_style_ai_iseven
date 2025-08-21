# Rust Bitcoin-Style AI is-even

A 🔥 BLAZING FAST 🔥 Rust library that uses Claude AI to check if a number is even, then returns the SHA256 hash of the response (Bitcoint Style).

whole new architecture

## Features

- Uses the latest and greatest Claude Opus 4.1 model (claude-opus-4-1-20250805) - Anthropic's most powerful AI model
- Bitcoint Style: Returns SHA256 hash of "True" or "False" 
- Both synchronous and asynchronous implementations
- Beautiful TUI in examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-bitcoin-style-ai-iseven = "0.1.0"
```

## Setup

1. Get an Anthropic API key from https://console.anthropic.com/
2. Set it as an environment variable:
   ```bash
   export ANTHROPIC_API_KEY="your-api-key-here"
   ```

## Usage

### Synchronous API

```rust
use rust_bitcoin_style_ai_iseven::AiIsEven;

fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")?;
    let ai_is_even = AiIsEven::new(api_key);
    
    let hash = ai_is_even.is_even(42)?;
    println!("is_even(42) = {}", hash);
    // Should print the SHA256 hash of "True"
    
    Ok(())
}
```

### Asynchronous API

```rust
use rust_bitcoin_style_ai_iseven::is_even_async;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")?;
    
    let hash = is_even_async(api_key, 42).await?;
    println!("is_even(42) = {}", hash);
    
    Ok(())
}
```

## Running Examples

```bash
# Set your API key
export ANTHROPIC_API_KEY="your-api-key-here"

# Run the synchronous example
cargo run --example basic_usage

# Run the async example  
cargo run --example async_usage
```

## Expected Output

- Even numbers return: `9df9a30bc5ecb14b6e991bd08220f367e652b3e773fb48eb637e0f1d09dc1540` (SHA256 of "True")
- Odd numbers return: `60a76841d99e06098c0ac3174f992dd58a0a5b5c9bc056e7f436e76b2b66638f` (SHA256 of "False")
