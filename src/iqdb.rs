use serde_derive::{Deserialize, Serialize};
use std::fmt;

pub fn get_iqdb(url: &str) -> ureq::Response {
    ureq::post("https://danbooru.iqdb.org/").send_form(&[
        ("file", ""),
        ("url", url),
        ("MAX_FILE_SIZE", "8388608"),
    ])
}

#[derive(SmartDefault, Debug, Deserialize, Serialize)]
pub struct Matches {
    pub match_type: MatchType,
    pub found: Vec<Match>,
}

impl Matches {
    pub fn string(&self) -> String {
        let mut out = String::new();

        out += match self.match_type {
            MatchType::Definite => "Found definite match\n",
            MatchType::Possible => "Found possible matches\n",
            MatchType::Unknown => "Found unknown matches\n",
        };

        let mut index = 1;
        for x in &self.found {
            out += (format!("#{}: {}\n", index, x)).as_ref();
            index += 1;
        }

        out
    }
}

impl fmt::Display for Matches {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.string())
    }
}

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
pub struct Match {
    pub link: String,
    pub img_link: String,
    pub similarity: isize,
    pub rating: MatchRating,
    pub size: MatchSize,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}% similarity {} {} | https:{} : {}",
            self.similarity, self.rating, self.size, self.link, self.img_link
        )
    }
}

#[derive(SmartDefault, Debug, Copy, Clone, Deserialize, Serialize)]
pub struct MatchSize {
    #[default = 0]
    pub width: usize,
    #[default = 0]
    pub height: usize,
    #[default = 0]
    pub bytes: usize,
}

impl fmt::Display for MatchSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}x{} ({} KB)",
            self.width,
            self.height,
            self.bytes / 1024
        )
    }
}

#[derive(SmartDefault, Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum MatchType {
    Possible,
    Definite,
    #[default]
    Unknown,
}

impl MatchType {
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

#[derive(SmartDefault, Debug, Copy, Clone, Deserialize, Serialize)]
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
