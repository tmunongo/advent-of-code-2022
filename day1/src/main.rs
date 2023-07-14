use color_eyre::eyre::Context;
use itertools::Itertools;

struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for file {:?}: {}", self.path, self.inner)
    }
}

fn main() -> color_eyre::Result<()> {
    // equivalent to writing, but now unwrap handles the two cases for us
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .max()
        .flatten()
        .unwrap_or_default();

    println!("{max:?}");

    Ok(())
}