use rust_bitcoin_style_ai_iseven::is_even_async;
use std::env;
use std::io::stdout;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    event::{self, Event, KeyCode},
};
use tokio::time::{sleep, Duration};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("Please set the ANTHROPIC_API_KEY environment variable");

    let test_numbers = vec![2, 3, 42, 69, 420, 1337];
    
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    terminal.clear()?;
    
    let mut results: Vec<(i32, String)> = Vec::new();
    let total = test_numbers.len();
    
    for (idx, number) in test_numbers.iter().enumerate() {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(f.size());
            
            let title = Paragraph::new("🎲 Rust Bitcoin-Style AI is-even (Async) 🎲")
                .style(Style::default().fg(ratatui::style::Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);
            
            let progress = (idx as f64 / total as f64) * 100.0;
            let gauge = Gauge::default()
                .block(Block::default().title("Progress").borders(Borders::ALL))
                .gauge_style(Style::default().fg(ratatui::style::Color::Yellow))
                .percent(progress as u16)
                .label(format!("Processing number {} of {}", idx + 1, total));
            f.render_widget(gauge, chunks[1]);
            
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Currently checking: ", Style::default().fg(ratatui::style::Color::Yellow)),
                    Span::styled(format!("{}", number), Style::default().fg(ratatui::style::Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
            ];
            
            for (num, hash) in &results {
                let is_even_hash = "3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18";
                let is_odd_hash = "60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe";
                
                let (symbol, color, hash_color) = if hash == is_even_hash {
                    ("✅", ratatui::style::Color::Green, ratatui::style::Color::Green)
                } else if hash == is_odd_hash {
                    ("❌", ratatui::style::Color::Red, ratatui::style::Color::Red)
                } else {
                    ("❓", ratatui::style::Color::Gray, ratatui::style::Color::Gray)
                };
                
                lines.push(Line::from(vec![
                    Span::raw(format!("{} is_even({:4}) -> ", symbol, num)),
                    Span::styled(hash.clone(), Style::default().fg(hash_color).add_modifier(Modifier::BOLD)),
                ]));
            }
            
            let results_widget = Paragraph::new(lines)
                .block(Block::default().title("Results").borders(Borders::ALL));
            f.render_widget(results_widget, chunks[2]);
            
            let footer = Paragraph::new("Press 'q' to quit after completion")
                .style(Style::default().fg(ratatui::style::Color::DarkGray))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[3]);
        })?;
        
        match is_even_async(api_key.clone(), *number).await {
            Ok(hash) => {
                results.push((*number, hash));
            }
            Err(e) => {
                results.push((*number, format!("Error: {}", e)));
            }
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(6),
                Constraint::Length(3),
            ])
            .split(f.size());
        
        let title = Paragraph::new("🎲 Rust Bitcoin-Style AI is-even (Async) - Complete! 🎲")
            .style(Style::default().fg(ratatui::style::Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);
        
        let mut lines = vec![];
        for (num, hash) in &results {
            let is_even_hash = "3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18";
            let is_odd_hash = "60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe";
            
            let (symbol, color, hash_color) = if hash == is_even_hash {
                ("✅", ratatui::style::Color::Green, ratatui::style::Color::Green)
            } else if hash == is_odd_hash {
                ("❌", ratatui::style::Color::Red, ratatui::style::Color::Red)
            } else {
                ("❓", ratatui::style::Color::Gray, ratatui::style::Color::Gray)
            };
            
            lines.push(Line::from(vec![
                Span::raw(format!("{} is_even({:4}) -> ", symbol, num)),
                Span::styled(hash.clone(), Style::default().fg(hash_color).add_modifier(Modifier::BOLD)),
            ]));
        }
        
        let results_widget = Paragraph::new(lines)
            .block(Block::default().title("Final Results").borders(Borders::ALL));
        f.render_widget(results_widget, chunks[1]);
        
        let reference_lines = vec![
            Line::from(vec![
                Span::styled("SHA256('True')  = ", Style::default().fg(ratatui::style::Color::White)),
                Span::styled("3cbc87c7681f34db4617feaa2c8801931bc5e42d8d0f560e756dd4cd92885f18", 
                    Style::default().fg(ratatui::style::Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("SHA256('False') = ", Style::default().fg(ratatui::style::Color::White)),
                Span::styled("60a33e6cf5151f2d52eddae9685cfa270426aa89d8dbc7dfb854606f1d1a40fe", 
                    Style::default().fg(ratatui::style::Color::Red).add_modifier(Modifier::BOLD)),
            ]),
        ];
        
        let reference = Paragraph::new(reference_lines)
            .block(Block::default().title("📖 Reference Hashes").borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(reference, chunks[2]);
        
        let footer = Paragraph::new("Press 'q' to quit")
            .style(Style::default().fg(ratatui::style::Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[3]);
    })?;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    
    disable_raw_mode()?;
    terminal.clear()?;
    
    Ok(())
}