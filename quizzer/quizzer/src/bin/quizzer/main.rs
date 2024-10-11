use std::{
    fs::File,
    io::{self, BufReader},
};

use add::add;
use anyhow::Ok;
use clap::{Parser, Subcommand};

mod add;
mod fake_io_wrapper;
mod io_wrapper;
mod play;
mod real_io_wrapper;

#[derive(Parser)]
#[command(
    name = "quizzer",
    version = "1.0",
    about = "quiz cli",
    arg_required_else_help = true
)]
struct Cli {
    #[arg(short, long)]
    file: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Play Command
    #[command(about = "start playing")]
    Play,

    /// Add Command
    #[command(about = "add a new question to a quiz")]
    Add,
}

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add) => {
            let mut file = File::create(&cli.file)?;
            add(&mut file, &mut stdin, &mut stdout)?;
            Ok(())
        } //start_editor()
        Some(Commands::Play) => {
            let mut file_reader = BufReader::new(File::open(cli.file)?);
            _ = play::play(&mut file_reader, &mut stdin, &mut stdout);
            Ok(())
        }
        _ => Ok(()),
    }
}
