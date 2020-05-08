use std::fmt;

pub fn get_iqdb(url: &str) -> ureq::Response {
    ureq::post("https://danbooru.iqdb.org/").send_form(&[
        ("file", ""),
        ("url", url),
        ("MAX_FILE_SIZE", "8388608"),
    ])
}

#[derive(SmartDefault, Debug)]
pub struct Matches {
    pub match_type: MatchType,
    pub found: Vec<Match>,
}

impl Matches {
    pub fn string(&self) -> String {
        let mut out = String::new();

        out += (format!("Type is: {}\n", &self.match_type)).as_ref();

        for x in &self.found {
            out += (format!("{}\n", x)).as_ref();
        }

        out
    }
}

impl fmt::Display for Matches {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.string())
    }
}

#[derive(SmartDefault, Debug, Clone)]
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

#[derive(SmartDefault, Debug, Copy, Clone)]
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
