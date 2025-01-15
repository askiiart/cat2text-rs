// this is a separate feature for *optional* dependencies to ensure clap doesn't get compiled if you're just using the library
use cat2text::{self, anybase, core};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::aot::{generate, Bash, Fish, PowerShell, Zsh};
use std::{io::stdout, time::Instant};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    ///Generate bash completions
    GenerateBashCompletions,
    ///Generate zsh completions
    GenerateZshCompletions,
    ///Generate fish completions
    GenerateFishCompletions,
    ///Generate PowerShell completions,
    GeneratePowershellCompletions,
    ///Encodes text/data to mrow~
    ///
    ///The default is base 4 text encoding, to match the original program, but it can encode either text or binary data in up to base 16 meows :3
    Encode {
        ///What base to encode using - up to base 16
        #[arg(short, long, default_value_t = 4)]
        base: u8,
        #[arg(long, default_value_t = false)]
        ///Whether to use byte encoding or English text encoding; text encoding can only handle a-z (lowercase)
        bytes: bool,
        text: String,
    },
    ///Decodes mrow~ to text/data
    ///
    ///The default is base 4 text encoding, to match the original program, but it can decode either text or binary data in up to base 16 meows :3
    Decode {
        ///What base to decode using - up to base 16
        #[arg(short, long, default_value_t = 4)]
        base: u8,
        #[arg(long, default_value_t = false)]
        ///Whether the input is using byte encoding or English text encoding
        bytes: bool,
        text: String,
    },
    Benchmark {
        ///What base to benchmark using - up to base 16
        #[arg(short, long, default_value_t = 4)]
        base: u8,
        ///Whether to use byte encoding or English text encoding; text encoding can only handle a-z (lowercase)
        #[arg(long, default_value_t = false)]
        bytes: bool,
        ///How many iterations to run each benchmark for
        #[arg(short, long, default_value_t = 1000)]
        iterations: u128,
        text: String,
    },
}

pub(crate) fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateBashCompletions => {
            generate(
                Bash,
                &mut Cli::command(),
                clap::crate_name!(),
                &mut stdout(),
            );
        }
        Commands::GenerateZshCompletions => {
            generate(Zsh, &mut Cli::command(), clap::crate_name!(), &mut stdout());
        }
        Commands::GenerateFishCompletions => {
            generate(
                Fish,
                &mut Cli::command(),
                clap::crate_name!(),
                &mut stdout(),
            );
        }
        Commands::GeneratePowershellCompletions => {
            generate(
                PowerShell,
                &mut Cli::command(),
                clap::crate_name!(),
                &mut stdout(),
            );
        }
        Commands::Encode { base, text, bytes } => {
            println!("{}", encode(base, text, bytes))
        }
        Commands::Decode { base, text, bytes } => {
            println!("{}", decode(base, text, bytes))
        }
        Commands::Benchmark {
            base,
            text,
            bytes,
            iterations,
        } => {
            let mut start = Instant::now();
            for _ in 0..iterations {
                let _ = encode(base, text.clone(), bytes);
            }
            let encode_time = start.elapsed();
            println!("Encode time: {}", encode_time.as_millis());

            start = Instant::now();
            for _ in 0..iterations {
                let _ = decode(base, text.clone(), bytes);
            }
            let decode_time = start.elapsed();

            println!("Decode time: {}", decode_time.as_millis());
        }
    }
}

fn encode(base: u8, text: String, bytes: bool) -> String {
    match bytes {
        true => {
            return format!(
                "{}",
                anybase::bytes::encode(
                    text.as_bytes(), // make the string into a Vec<u8>
                    base as u32,
                    core::bytes::char_length(base as u32)
                )
            );
        }
        false => {
            return format!(
                "{}",
                anybase::encode(text, base as u32, core::char_length(base as u32))
            );
        }
    }
}

fn decode(base: u8, text: String, bytes: bool) -> String {
    match bytes {
        true => {
            return anybase::bytes::decode(
                text,
                base as u32,
                core::bytes::char_length(base as u32),
            )
            .into_iter()
            .map(|item| item.to_string() + " ")
            .collect::<String>()
            .trim()
            .to_string();
        }
        false => {
            return anybase::decode(text, base as u32, core::char_length(base as u32));
        }
    }
}
