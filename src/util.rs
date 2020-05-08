use crate::iqdb::{get_iqdb, Matches};
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt;

fn check_url(url: &str) -> bool {
    let head = ureq::head(url).call();

    head.header("content-type").unwrap_or("").contains("image")
}

pub fn build_match(url: &str) -> Result<Matches, Box<dyn Error>> {
    if !check_url(url) {
        return Err(Box::new(MatchError(format!("invalid url: {}", url))));
    }

    let iqdb = get_iqdb(url);

    println!("Debug {:#?}", iqdb);

    if !iqdb.ok() {
        return Err(Box::new(MatchError(format!(
            "got error {} from iqdb: {}",
            iqdb.status(),
            iqdb.status_text()
        ))));
    }

    Ok(Matches::default())
}

#[derive(Debug, Clone)]
struct MatchError(String);

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error getting matches: {}", self.0)
    }
}

impl Error for MatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
