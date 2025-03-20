#![allow(dead_code)]

//! An example CLI that uses `MaybeStdinFromSource`
//! to parse a string differently depending on the source
//!
//! Example usage:
//! ```sh
//! # via stdin
//! $ printf "banana\napple\n" | cargo run --example from_source
//!
//! # or equivalently
//! $ cat <<EOF | cargo run --example from_source
//! banana
//! apple
//! orange
//! kiwi
//! EOF
//! # via command line argument
//! $ cargo run --example from_source -- banana,apple,orange,kiwi
//! ```

use clap::Parser;
use clap_stdin::{FromSource, MaybeStdinFromSource, Source, StdinError};

const MAX_FRUITS: usize = 3;

#[derive(Debug, Clone)]
pub struct Fruits(Vec<String>);

impl FromSource for Fruits {
    type Err = StdinError;

    fn from_source(source: Source) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        match source {
            Source::Stdin(stdin) => {
                let fruits = stdin
                    .lines()?
                    .map(|r| r.map(String::from))
                    .collect::<Result<Vec<String>, _>>()?;
                Ok(Fruits(fruits))
            }
            Source::Arg(arg) => {
                let fruits = arg.split(",").map(String::from).collect::<Vec<_>>();
                Ok(Fruits(fruits))
            }
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Parsed user from json, provided via a filepath (or leave blank to read from stdin)
    #[arg(default_value = "-")]
    fruits: MaybeStdinFromSource<Fruits>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    eprintln!("{:?}", args.fruits);
    Ok(())
}
