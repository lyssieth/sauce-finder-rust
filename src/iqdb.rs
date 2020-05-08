use serde_json::json;
use std::fmt;

pub fn get_iqdb(url: &str) -> ureq::Response {
    let data = json!({
        "file": "(binary)",
        "url": url
    });
    ureq::post(url).send_json(data)
}

#[derive(SmartDefault, Debug)]
pub struct Matches<'a> {
    match_type: MatchType,
    found: Vec<Match<'a>>,
}

#[derive(SmartDefault, Debug, Copy, Clone)]
pub struct Match<'a> {
    link: &'a str,
    similarity: &'a str,
    rating: MatchRating,
    size: MatchSize,
}

impl<'a> fmt::Display for Match<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}% similarity {} {} | {}",
            self.similarity, self.rating, self.size, self.link
        )
    }
}

#[derive(SmartDefault, Debug, Copy, Clone)]
pub struct MatchSize {
    #[default = 0]
    width: usize,
    #[default = 0]
    height: usize,
}

impl fmt::Display for MatchSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(SmartDefault, Debug, Copy, Clone)]
pub enum MatchType {
    Possible,
    Definite,
    #[default]
    Unknown,
}

impl MatchType {
    pub fn get_from_string(value: &str) -> Self {
        let val = value.to_lowercase();

        if val.contains("possible") {
            Self::Possible
        } else if val.contains("definite") {
            Self::Definite
        } else {
            Self::Unknown
        }
    }

    pub fn str<'a>(self) -> &'a str {
        match self {
            Self::Possible => "Possible",
            Self::Definite => "Definite",
            Self::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for MatchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

#[derive(SmartDefault, Debug, Copy, Clone)]
pub enum MatchRating {
    Safe,
    Questionable,
    Explicit,
    #[default]
    Unknown,
}

impl MatchRating {
    pub fn get_from_string(value: &str) -> Self {
        let val = value.to_lowercase();

        if val.contains("safe") {
            Self::Safe
        } else if val.contains("questionable") {
            Self::Questionable
        } else if val.contains("explicit") {
            Self::Explicit
        } else {
            Self::Unknown
        }
    }

    pub fn str<'a>(self) -> &'a str {
        match self {
            Self::Safe => "Safe",
            Self::Questionable => "Questionable",
            Self::Explicit => "Explicit",
            Self::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for MatchRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.str())
    }
}
