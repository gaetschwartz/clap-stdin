#![allow(dead_code)]

//! An example CLI that uses `MaybeStdinVec`
//! to parse a list of fruits from a source.
//! When provided via stdin, the fruits are expected to be separated by newlines.
//! When provided via command line argument, the fruits are expected
//! to be separated by commas, but you can choose any delimiter
//! by providing it as a const generic.
//!
//! Example usage:
//! ```sh
//! # via stdin
//! $ printf "banana\napple\n" | cargo run --example from_source_vec
//!
//! # or equivalently
//! $ cat <<EOF | cargo run --example from_source_vec
//! banana
//! apple
//! EOF
//! # via command line argument
//! $ cargo run --example from_source_vec -- banana-apple
//! ```

use clap::Parser;
use clap_stdin::MaybeStdinVec;

#[derive(Debug, Parser)]
struct Args {
    /// Parsed user from json, provided via a filepath (or leave blank to read from stdin)
    #[arg(default_value = "-")]
    fruits: MaybeStdinVec<String, '-'>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    eprintln!("{:?}", args.fruits);
    Ok(())
}
