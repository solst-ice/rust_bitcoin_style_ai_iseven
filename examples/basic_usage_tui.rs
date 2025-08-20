use rust_bitcoin_style_ai_iseven::AiIsEven;
use std::env;
use std::io::{self, stdout};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    cursor,
};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("Please set the ANTHROPIC_API_KEY environment variable");

    let ai_is_even = AiIsEven::new(api_key);

    let test_numbers = vec![2, 3, 42, 69, 420, 1337];
    
    let mut stdout = stdout();
    
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("╔══════════════════════════════════════════════════════════════════════════════╗\n"),
        Print("║                    🎲 Rust Bitcoin-Style AI is-even 🎲                        ║\n"),
        Print("╚══════════════════════════════════════════════════════════════════════════════╝\n"),
        ResetColor
    )?;
    
    println!();

    for (idx, number) in test_numbers.iter().enumerate() {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("🔍 Checking if {} is even", number)),
            ResetColor
        )?;
        
        for _ in 0..3 {
            thread::sleep(Duration::from_millis(200));
            print!(".");
            io::Write::flush(&mut stdout)?;
        }
        println!();
        
        match ai_is_even.is_even(*number) {
            Ok(hash) => {
                let is_even_hash = "3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18";
                let is_odd_hash = "60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe";
                
                if hash == is_even_hash {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Green),
                        Print(format!("✅ is_even({:4}) -> ", number)),
                        SetForegroundColor(Color::Green),
                        Print(format!("{}", hash.bold())),
                        ResetColor,
                        Print("\n")
                    )?;
                } else if hash == is_odd_hash {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        Print(format!("❌ is_even({:4}) -> ", number)),
                        SetForegroundColor(Color::Red),
                        Print(format!("{}", hash.bold())),
                        ResetColor,
                        Print("\n")
                    )?;
                } else {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::White),
                        Print(format!("❓ is_even({:4}) -> {}", number, hash)),
                        ResetColor,
                        Print("\n")
                    )?;
                }
            }
            Err(e) => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Magenta),
                    Print(format!("⚠️  Error checking {}: {}\n", number, e)),
                    ResetColor
                )?;
            }
        }
        
        if idx < test_numbers.len() - 1 {
            println!();
        }
    }

    println!("\n");
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("╔══════════════════════════════════════════════════════════════════════════════╗\n"),
        Print("║                              📖 Reference Hashes 📖                           ║\n"),
        Print("╠══════════════════════════════════════════════════════════════════════════════╣\n"),
        ResetColor
    )?;
    
    execute!(
        stdout,
        Print("║ "),
        SetForegroundColor(Color::Green),
        Print("SHA256('True')  = 3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18"),
        ResetColor,
        Print(" ║\n")
    )?;
    
    execute!(
        stdout,
        Print("║ "),
        SetForegroundColor(Color::Red),
        Print("SHA256('False') = 60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe"),
        ResetColor,
        Print(" ║\n")
    )?;
    
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("╚══════════════════════════════════════════════════════════════════════════════╝\n"),
        ResetColor
    )?;

    Ok(())
}