use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline, Wrap,
    },
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use tokio::time::interval;

mod rpc;
mod state;

use rpc::*;
use state::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CLI STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "asf-monitor")]
#[command(author = "Ëtrid Foundation")]
#[command(version = "0.1.0")]
#[command(about = "Real-time monitoring dashboard for ËTRID ASF validators")]
struct Cli {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "http://localhost:9944")]
    rpc: String,

    /// Validator address to monitor
    #[arg(short, long)]
    validator: Option<String>,

    /// Update interval in seconds
    #[arg(short = 'i', long, default_value = "2")]
    interval: u64,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        env_logger::init();
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(cli.rpc.clone(), cli.validator, cli.interval);

    // Run the app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// APPLICATION RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => app.refresh().await?,
                    KeyCode::Char('c') => app.clear_history(),
                    KeyCode::Up => app.scroll_up(),
                    KeyCode::Down => app.scroll_down(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick().await?;
            last_tick = Instant::now();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// UI RENDERING
// ═══════════════════════════════════════════════════════════════════════════════

fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(7),  // Status overview
            Constraint::Length(10), // Metrics
            Constraint::Min(8),     // Activity log
            Constraint::Length(3),  // Footer
        ])
        .split(f.size());

    // Header
    render_header(f, chunks[0], app);

    // Status overview
    render_status(f, chunks[1], app);

    // Metrics
    render_metrics(f, chunks[2], app);

    // Activity log
    render_activity_log(f, chunks[3], app);

    // Footer
    render_footer(f, chunks[4]);
}

fn render_header<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let title = vec![
        Span::styled("ËTRID ASF ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("Validator Monitor", Style::default().fg(Color::White)),
    ];

    let subtitle = if let Some(ref validator) = app.validator_address {
        format!("Monitoring: {}", validator)
    } else {
        "Monitoring: All Validators".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(vec![
        Line::from(title),
        Line::from(Span::raw(subtitle)),
    ])
    .block(block)
    .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_status<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(area);

    // Network status
    let network_items = vec![
        ListItem::new(format!("Chain: {}", app.state.chain_name)),
        ListItem::new(format!("Block: #{}", app.state.current_block)),
        ListItem::new(format!(
            "Finality: Level {}",
            app.state.finality_level
        )),
        ListItem::new(format!("Peers: {}", app.state.peer_count)),
        ListItem::new(format!(
            "Status: {}",
            if app.state.is_syncing { "Syncing" } else { "Synced" }
        )),
    ];

    let network_list = List::new(network_items)
        .block(
            Block::default()
                .title("Network")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(network_list, chunks[0]);

    // Validator status
    let validator_items = vec![
        ListItem::new(format!("Active: {}", if app.state.is_active { "Yes" } else { "No" })),
        ListItem::new(format!("Committee: {}", if app.state.in_committee { "Yes" } else { "No" })),
        ListItem::new(format!("Votes Cast: {}", app.state.votes_cast)),
        ListItem::new(format!("Certificates: {}", app.state.certificates_issued)),
        ListItem::new(format!("Slashed: {}", if app.state.is_slashed { "YES" } else { "No" })),
    ];

    let validator_color = if app.state.is_slashed {
        Color::Red
    } else if app.state.is_active {
        Color::Green
    } else {
        Color::Yellow
    };

    let validator_list = List::new(validator_items)
        .block(
            Block::default()
                .title("Validator")
                .borders(Borders::ALL)
                .style(Style::default().fg(validator_color)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(validator_list, chunks[1]);

    // Performance
    let uptime_percent = app.state.uptime_percentage;
    let performance_items = vec![
        ListItem::new(format!("Uptime: {}%", uptime_percent)),
        ListItem::new(format!("Blocks Signed: {}", app.state.blocks_signed)),
        ListItem::new(format!("Missed Blocks: {}", app.state.missed_blocks)),
        ListItem::new(format!("Health Score: {}/100", app.state.health_score)),
        ListItem::new(format!("Reputation: {}/100", app.state.reputation)),
    ];

    let perf_color = if uptime_percent >= 95 {
        Color::Green
    } else if uptime_percent >= 80 {
        Color::Yellow
    } else {
        Color::Red
    };

    let performance_list = List::new(performance_items)
        .block(
            Block::default()
                .title("Performance")
                .borders(Borders::ALL)
                .style(Style::default().fg(perf_color)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(performance_list, chunks[2]);
}

fn render_metrics<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(4)])
        .split(area);

    // Health score gauge
    let health_ratio = app.state.health_score as f64 / 100.0;
    let health_gauge = Gauge::default()
        .block(Block::default().title("Health Score").borders(Borders::ALL))
        .gauge_style(Style::default().fg(health_color(app.state.health_score)))
        .ratio(health_ratio);

    f.render_widget(health_gauge, chunks[0]);

    // Uptime gauge
    let uptime_ratio = app.state.uptime_percentage as f64 / 100.0;
    let uptime_gauge = Gauge::default()
        .block(Block::default().title("Uptime").borders(Borders::ALL))
        .gauge_style(Style::default().fg(uptime_color(app.state.uptime_percentage)))
        .ratio(uptime_ratio);

    f.render_widget(uptime_gauge, chunks[1]);

    // Block production sparkline
    let sparkline = Sparkline::default()
        .block(Block::default().title("Block Production (last 60)").borders(Borders::ALL))
        .data(&app.state.block_history)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, chunks[2]);
}

fn render_activity_log<B: Backend>(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .state
        .activity_log
        .iter()
        .rev()
        .skip(app.scroll_offset)
        .take(area.height.saturating_sub(2) as usize)
        .map(|log_entry| {
            let style = match log_entry.level.as_str() {
                "ERROR" => Style::default().fg(Color::Red),
                "WARN" => Style::default().fg(Color::Yellow),
                "INFO" => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::White),
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("[{}] ", log_entry.timestamp),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("[{}] ", log_entry.level),
                    style,
                ),
                Span::raw(&log_entry.message),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!("Activity Log ({} entries)", app.state.activity_log.len()))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Magenta)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_footer<B: Backend>(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Quit | "),
        Span::styled("r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Refresh | "),
        Span::styled("c", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Clear History | "),
        Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Scroll"),
    ];

    let paragraph = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn health_color(score: u8) -> Color {
    match score {
        90..=100 => Color::Green,
        70..=89 => Color::Yellow,
        50..=69 => Color::LightYellow,
        30..=49 => Color::LightRed,
        _ => Color::Red,
    }
}

fn uptime_color(uptime: u8) -> Color {
    match uptime {
        95..=100 => Color::Green,
        80..=94 => Color::Yellow,
        _ => Color::Red,
    }
}
