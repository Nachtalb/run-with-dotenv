use clap::Parser;
use color_print::ceprintln;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(about = "Run command with .env vars (unexported)")]
struct Args {
    /// Path to .env file
    #[arg(short, long, default_value = ".env")]
    file: PathBuf,

    /// Command to run
    command: String,

    /// Command args
    args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut env_vars = std::env::vars().collect::<Vec<_>>();

    let output = match dotenvy::from_path(&args.file) {
        Ok(_) => {
            for (key, value) in dotenvy::vars() {
                env_vars.push((key, value));
            }
            Command::new(&args.command)
                .args(args.args)
                .envs(env_vars)
                .output()?
        }
        Err(_) => {
            ceprintln!("<red>{} not found</red>", args.file.display());
            Command::new(&args.command).args(args.args).output()?
        }
    };

    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    std::process::exit(output.status.code().unwrap_or(1));
}
