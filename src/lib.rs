#![doc = include_str!("../README.md")]

use std::io::{self, BufRead, Read, StdinLock};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
mod maybe_stdin;
pub use maybe_stdin::MaybeStdin;
mod maybe_stdin_from_source;
pub use maybe_stdin_from_source::FromSource;
pub use maybe_stdin_from_source::MaybeStdinFromSource;
pub use maybe_stdin_from_source::MaybeStdinVec;

mod file_or_stdin;
pub use file_or_stdin::FileOrStdin;

static STDIN_HAS_BEEN_READ: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
pub enum StdinError {
    #[error("stdin read from more than once")]
    StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] io::Error),
    #[error("unable to parse from_str: {0}")]
    FromStr(String),
    #[error("unable to parse from_source: {0}")]
    FromSource(String),
}

/// Source of the value contents will be either from `stdin` or a CLI arg provided value
#[derive(Clone)]
pub enum Source {
    Stdin(Stdin),
    Arg(String),
}

/// Stdin source, which can be used to read from `stdin`. DO NOT read from stdin yourself, use `Stdin.read()` instead.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Stdin;

impl Source {
    pub(crate) fn into_reader(self) -> Result<impl std::io::Read, StdinError> {
        let input: Box<dyn std::io::Read + 'static> = match self {
            Source::Stdin(_) => {
                if STDIN_HAS_BEEN_READ.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_READ.store(true, std::sync::atomic::Ordering::SeqCst);
                Box::new(std::io::stdin())
            }
            Source::Arg(filepath) => {
                let f = std::fs::File::open(filepath)?;
                Box::new(f)
            }
        };
        Ok(input)
    }

    pub(crate) fn get_value(self) -> Result<String, StdinError> {
        match self {
            Source::Stdin(_) => {
                if STDIN_HAS_BEEN_READ.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_READ.store(true, std::sync::atomic::Ordering::SeqCst);
                let stdin = io::stdin();
                let mut input = String::new();
                stdin.lock().read_to_string(&mut input)?;
                Ok(input)
            }
            Source::Arg(value) => Ok(value),
        }
    }
}

impl FromStr for Source {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdin(Stdin)),
            arg => Ok(Self::Arg(arg.to_owned())),
        }
    }
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Stdin(_) => write!(f, "stdin"),
            Source::Arg(v) => v.fmt(f),
        }
    }
}

impl Stdin {
    /// Read from stdin. Use this method to read from stdin and DO NOT read from stdin yourself.
    pub fn read_string(&self) -> Result<String, StdinError> {
        Source::Stdin(Stdin).get_value()
    }

    pub fn lines(&self) -> Result<io::Lines<StdinLock>, StdinError> {
        if STDIN_HAS_BEEN_READ.load(std::sync::atomic::Ordering::Acquire) {
            return Err(StdinError::StdInRepeatedUse);
        };
        STDIN_HAS_BEEN_READ.store(true, std::sync::atomic::Ordering::SeqCst);
        let stdin = io::stdin();
        return Ok(stdin.lock().lines());
    }
}
