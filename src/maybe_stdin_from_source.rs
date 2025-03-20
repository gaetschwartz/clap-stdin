use core::panic;
use std::str::FromStr;

use super::{Source, StdinError};

/// Wrapper struct to parse arg values from `stdin`
///
/// `MaybeStdin` can wrap any type that matches the trait bounds for `Arg`: `FromStr` and `Clone`
/// ```rust
/// use std::path::PathBuf;
/// use clap::Parser;
/// use clap_stdin::{MaybeStdinFromSource, StdinError, Source, FromSource};
///
/// #[derive(Debug, Parser)]
/// struct Args {
///     path: MaybeStdinFromSource<Fruits>,
/// }
///
/// #[derive(Debug, Clone)]
/// pub struct Fruits(Vec<String>);
///
/// impl FromSource for Fruits {
///     type Err = StdinError;
///
///     fn from_source(source: Source) -> Result<Self, Self::Err>
///     where
///         Self: Sized,
///     {
///         match source {
///             Source::Stdin(stdin) => {
///                 let fruits = stdin
///                     .lines()?
///                     .map(|r| r.map(String::from))
///                     .collect::<Result<Vec<String>, _>>()?;
///                 Ok(Fruits(fruits))
///             }
///             Source::Arg(arg) => {
///                 let fruits = arg.split(",").map(String::from).collect::<Vec<_>>();
///                 Ok(Fruits(fruits))
///             }
///         }
///     }
/// }
///
/// if let Ok(args) = Args::try_parse() {
///     println!("fruits={:?}", args.path);
/// }
/// ```
///
/// ```sh
/// $ pwd | ./example -
/// /current/working/dir
/// ```

#[derive(Clone)]
pub struct MaybeStdinFromSource<T> {
    inner: T,
    is_stdin: bool,
}

impl<T> MaybeStdinFromSource<T> {
    pub fn is_stdin(&self) -> bool {
        self.is_stdin
    }
}

pub trait FromSource {
    type Err;

    fn from_source(source: Source) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl<T> FromStr for MaybeStdinFromSource<T>
where
    T: FromSource,
    T::Err: std::fmt::Display,
{
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Source::from_str(s)?;
        let is_stdin = matches!(source, Source::Stdin(_));
        T::from_source(source)
            .map_err(|e| StdinError::FromStr(format!("{e}")))
            .map(|val| Self {
                inner: val,
                is_stdin,
            })
    }
}

impl<T> MaybeStdinFromSource<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::fmt::Display for MaybeStdinFromSource<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::fmt::Debug for MaybeStdinFromSource<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> std::ops::Deref for MaybeStdinFromSource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for MaybeStdinFromSource<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Clone)]
pub struct MaybeStdinVec<T, const D: char = ','> {
    inner: Vec<T>,
    is_stdin: bool,
}

impl<T, const D: char> MaybeStdinVec<T, D> {
    pub fn is_stdin(&self) -> bool {
        self.is_stdin
    }
}

impl<T, const D: char> FromStr for MaybeStdinVec<T, D>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Source::from_str(s)?;
        let is_stdin = matches!(source, Source::Stdin(_));
        if is_stdin {
            source
                .get_value()?
                .trim()
                .lines()
                .map(|s| T::from_str(s).map_err(|e| StdinError::FromStr(format!("{e}"))))
                .collect::<Result<Vec<T>, _>>()
                .map(|inner| Self { inner, is_stdin })
        } else {
            source
                .get_value()?
                .trim()
                .split(D)
                .map(|s| T::from_str(s).map_err(|e| StdinError::FromStr(format!("{e}"))))
                .collect::<Result<Vec<T>, _>>()
                .map(|inner| Self { inner, is_stdin })
        }
    }
}

impl<T, const D: char> FromIterator<String> for MaybeStdinVec<T, D>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        iter.into_iter()
            .map(|s| T::from_str(&s))
            .collect::<Result<Vec<T>, _>>()
            .map(|inner| Self {
                inner,
                is_stdin: false,
            })
            .unwrap_or_else(|error| panic!("Failed to parse input: {error}"))
    }
}

impl<T, const D: char> MaybeStdinVec<T, D> {
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}

impl<T, const D: char> std::fmt::Debug for MaybeStdinVec<T, D>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T, const D: char> std::ops::Deref for MaybeStdinVec<T, D> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, const D: char> std::ops::DerefMut for MaybeStdinVec<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
