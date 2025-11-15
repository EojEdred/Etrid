use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process;
use sysinfo::{System, SystemExt, ProcessExt, CpuExt, DiskExt};

mod checks;
mod rpc;

use checks::*;
use rpc::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CLI STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "asf-health")]
#[command(author = "Ëtrid Foundation")]
#[command(version = "0.1.0")]
#[command(about = "Health check utility for ËTRID ASF validators")]
struct Cli {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "http://localhost:9944")]
    rpc: String,

    /// Validator address to check
    #[arg(short, long)]
    validator: Option<String>,

    /// Output format (text, json)
    #[arg(short = 'f', long, default_value = "text")]
    format: OutputFormat,

    /// Output file (prints to stdout if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Exit with non-zero code on any failures
    #[arg(short = 'e', long)]
    exit_on_failure: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(anyhow::anyhow!("Invalid format. Use 'text' or 'json'")),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut health_report = HealthReport::new();

    println!("{}", "Running ËTRID ASF Validator Health Checks...".cyan().bold());
    println!();

    // 1. Node connectivity check
    print_check_header("Node Connectivity");
    let connectivity = check_node_connectivity(&cli.rpc, cli.verbose).await;
    health_report.add_check(connectivity);

    // 2. Key accessibility check
    if let Some(ref validator) = cli.validator {
        print_check_header("Key Accessibility");
        let key_check = check_key_accessibility(validator, cli.verbose).await;
        health_report.add_check(key_check);
    }

    // 3. Stake check
    if let Some(ref validator) = cli.validator {
        print_check_header("Stake Verification");
        let stake_check = check_stake(&cli.rpc, validator, cli.verbose).await;
        health_report.add_check(stake_check);
    }

    // 4. Slashing status check
    if let Some(ref validator) = cli.validator {
        print_check_header("Slashing Status");
        let slash_check = check_slashing_status(&cli.rpc, validator, cli.verbose).await;
        health_report.add_check(slash_check);
    }

    // 5. P2P peer count check
    print_check_header("P2P Peer Count");
    let peer_check = check_peer_count(&cli.rpc, cli.verbose).await;
    health_report.add_check(peer_check);

    // 6. Sync status check
    print_check_header("Sync Status");
    let sync_check = check_sync_status(&cli.rpc, cli.verbose).await;
    health_report.add_check(sync_check);

    // 7. System resources check
    print_check_header("System Resources");
    let resource_check = check_system_resources(cli.verbose).await;
    health_report.add_check(resource_check);

    // Print summary
    println!();
    print_summary(&health_report);

    // Output results
    let output_content = match cli.format {
        OutputFormat::Text => format_text_report(&health_report),
        OutputFormat::Json => format_json_report(&health_report)?,
    };

    if let Some(output_path) = cli.output {
        std::fs::write(&output_path, output_content)
            .context("Failed to write output file")?;
        println!("\n{} Report saved to: {}", "✓".green(), output_path.display());
    }

    // Exit with appropriate code
    if cli.exit_on_failure && !health_report.all_passed() {
        process::exit(1);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH REPORT STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    checks: Vec<HealthCheck>,
    timestamp: String,
}

impl HealthReport {
    fn new() -> Self {
        Self {
            checks: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn add_check(&mut self, check: HealthCheck) {
        self.checks.push(check);
    }

    fn all_passed(&self) -> bool {
        self.checks.iter().all(|c| c.status == CheckStatus::Pass)
    }

    fn passed_count(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Pass).count()
    }

    fn failed_count(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Fail).count()
    }

    fn warning_count(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Warning).count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    name: String,
    status: CheckStatus,
    message: String,
    details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

// ═══════════════════════════════════════════════════════════════════════════════
// OUTPUT FORMATTING
// ═══════════════════════════════════════════════════════════════════════════════

fn print_check_header(name: &str) {
    println!("{}", format!("├─ {}", name).cyan());
}

fn print_check_result(check: &HealthCheck) {
    let (symbol, color_fn): (&str, fn(&str) -> colored::ColoredString) = match check.status {
        CheckStatus::Pass => ("✓", |s: &str| s.green()),
        CheckStatus::Warning => ("⚠", |s: &str| s.yellow()),
        CheckStatus::Fail => ("✗", |s: &str| s.red()),
    };

    println!("   {} {}", color_fn(symbol), color_fn(&check.message));
}

fn print_summary(report: &HealthReport) {
    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "Health Check Summary".cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());

    let total = report.checks.len();
    let passed = report.passed_count();
    let failed = report.failed_count();
    let warnings = report.warning_count();

    println!("Total Checks: {}", total);
    println!("  {} Passed: {}", "✓".green(), passed);
    if warnings > 0 {
        println!("  {} Warnings: {}", "⚠".yellow(), warnings);
    }
    if failed > 0 {
        println!("  {} Failed: {}", "✗".red(), failed);
    }

    println!();
    if report.all_passed() {
        println!("{}", "✓ All health checks passed!".green().bold());
    } else if failed > 0 {
        println!("{}", "✗ Some health checks failed!".red().bold());
    } else {
        println!("{}", "⚠ Health checks completed with warnings".yellow().bold());
    }
}

fn format_text_report(report: &HealthReport) -> String {
    let mut output = String::new();

    output.push_str(&format!("ËTRID ASF Validator Health Report\n"));
    output.push_str(&format!("Generated: {}\n\n", report.timestamp));

    for check in &report.checks {
        let status_str = match check.status {
            CheckStatus::Pass => "PASS",
            CheckStatus::Warning => "WARN",
            CheckStatus::Fail => "FAIL",
        };

        output.push_str(&format!("[{}] {}: {}\n", status_str, check.name, check.message));

        if let Some(ref details) = check.details {
            output.push_str(&format!("  Details: {}\n", serde_json::to_string_pretty(details).unwrap()));
        }
        output.push('\n');
    }

    output
}

fn format_json_report(report: &HealthReport) -> Result<String> {
    Ok(serde_json::to_string_pretty(report)?)
}

// Add chrono dependency usage
use chrono;
