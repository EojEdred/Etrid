use clap::CommandFactory;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "etrid-cli", version, about = "Ëtrid CLI Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completions {
        #[arg(short, long, default_value = "bash")]
        shell: String,
    },
}

impl Cli {
    pub fn generate_completions() {
        use clap_complete::{generate_to, shells};
        use std::path::Path;

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let path = Path::new(&out_dir);

        generate_to(shells::Bash, &mut Cli::command(), "etrid", path).unwrap();
        generate_to(shells::Zsh, &mut Cli::command(), "etrid", path).unwrap();
        generate_to(shells::Fish, &mut Cli::command(), "etrid", path).unwrap();
        generate_to(shells::PowerShell, &mut Cli::command(), "etrid", path).unwrap();
        generate_to(shells::Elvish, &mut Cli::command(), "etrid", path).unwrap();
    }
}
