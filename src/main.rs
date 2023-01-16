use clap::{Parser, Subcommand};
use il_tz::{generate, str2tz, tz2str, validate, MAX_TZ_RANGE};

#[derive(Parser)]
#[command(about = "Validate Israeli ID numbers (TZ)")]
struct Cli {
    #[arg(value_name = "TZ")]
    /// A list of ID numbers (TZ) to validate
    tz: Option<Vec<String>>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a list of valid TZ values
    Generate {
        /// The start of the range
        #[arg(value_name = "START")]
        start: usize,
        /// The end of the range
        #[arg(value_name = "END")]
        end: usize,
    },
}

fn main() {
    let args = Cli::parse();
    if let Some(tz_list) = args.tz {
        for tz_str in tz_list {
            let tz = str2tz(&tz_str);
            match tz {
                Ok(tz) => println!("{} {}", tz2str(&tz), validate(&tz)),
                Err(_) => println!("{} false", tz_str),
            }
        }
    }

    if let Some(command) = args.command {
        match command {
            Commands::Generate { start, end } => {
                if start > end {
                    eprintln!("START must be less than END");
                    eprintln!("Run with --help for more information");
                    std::process::exit(1);
                }
                if end > MAX_TZ_RANGE {
                    eprintln!("End must be less than {}", MAX_TZ_RANGE);
                    eprintln!("Run with --help for more information");
                    std::process::exit(1);
                }
                let tz_list = generate(start, end);
                for tz in tz_list {
                    println!("{}", tz2str(&tz));
                }
            }
        }
    }
}
