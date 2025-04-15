//! BudouX CLI
//!
//! Command-line interface for BudouX Japanese text segmentation

#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Text to segment
    #[arg(required = true)]
    text: String,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    #[cfg(feature = "cli")]
    {
        let cli = Cli::parse();
        let parser = budoux_rust_wrapper::load_default_japanese_parser();
        let result = parser.parse(&cli.text);

        match cli.format.as_str() {
            "json" => {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            _ => {
                for chunk in result {
                    println!("{}", chunk);
                }
            }
        }
    }

    #[cfg(not(feature = "cli"))]
    {
        println!("This binary requires the 'cli' feature to be enabled.");
        println!("Please rebuild with: cargo build --features cli");
    }
}
