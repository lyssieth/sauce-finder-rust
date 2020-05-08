use crate::iqdb::Matches;
use std::error::Error;
use std::fmt;

fn check_url(url: &str) -> bool {
    let head = ureq::head(url).call();

    head.header("content-type").unwrap_or("").contains("image")
}

pub fn build_match(url: &str) -> Result<Matches, Box<dyn Error>> {
    if !check_url(url) {
        return Err(Box::new(BuildError(format!("invalid url: {}", url))));
    }

    Ok(Matches::default())
}

#[derive(Debug, Clone)]
struct BuildError(String);

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
