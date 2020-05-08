use crate::iqdb::{get_iqdb, Match, MatchRating, MatchSize, MatchType, Matches};
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt;

fn check_url(url: &str) -> bool {
    let head = ureq::head(url).call();

    head.header("content-type").unwrap_or("").contains("image")
}

pub fn get_direct_from_booru(url: &str) -> Result<String, Box<dyn Error>> {
    let res = ureq::get(url).call();

    if !res.ok() {
        return Err(Box::new(MatchError(format!(
            "got error {} from booru: {}",
            res.status(),
            res.status_text()
        ))));
    }
    let html = Html::parse_document(res.into_string().unwrap().as_ref());

    let img = html
        .select(&Selector::parse("#image").unwrap())
        .next()
        .unwrap()
        .value();

    Ok(img.attr("src").unwrap().to_string())
}

pub fn build_match(url: &str) -> Result<Matches, Box<dyn Error>> {
    if !check_url(url) {
        return Err(Box::new(MatchError(format!("invalid url: {}", url))));
    }

    let iqdb = get_iqdb(url);

    if !iqdb.ok() {
        return Err(Box::new(MatchError(format!(
            "got error {} from iqdb: {}",
            iqdb.status(),
            iqdb.status_text()
        ))));
    }

    let mut output: Matches = Matches::default();
    let html = Html::parse_document(iqdb.into_string().unwrap().as_ref());

    let possible = if html.select(&Selectors::no_match()).count() > 0usize {
        output.match_type = MatchType::Possible;
        true
    } else {
        output.match_type = MatchType::Definite;
        false
    };

    let mut i = 0;
    for x in html.select(&Selectors::matches()) {
        if (possible && i < 2) || (!possible && i < 1) {
            i += 1;
            continue;
        }

        let mut item: Match = Match::default();
        let post_url = x.select(&Selectors::match_url()).next().unwrap();
        item.link = post_url.value().attr("href").unwrap().to_string();

        item.img_link = get_direct_from_booru(post_url.value().attr("href").unwrap()).unwrap();

        let post_data_bad = x
            .select(&Selectors::match_post_data())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let post_data: Vec<&str> = post_data_bad.split(' ').collect();
        let size_data: Vec<&str> = post_data[0].split('×').collect();
        let size = MatchSize {
            width: size_data[0].parse().unwrap(),
            height: size_data[1].parse().unwrap(),
            bytes: get_size(post_url.value().attr("href").unwrap()),
        };
        let rating = MatchRating::get_from_string(post_data[1]);

        item.size = size;
        item.rating = rating;

        let post_similarity = x
            .select(&Selectors::match_post_similarity())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let post_similarity_bad = post_similarity.split(' ').collect::<Vec<_>>();
        let similarity: String = post_similarity_bad[0].replace("%", "");
        item.similarity = similarity
            .replace("%", "")
            .into_boxed_str()
            .parse()
            .unwrap();

        output.found.push(item);
    }

    Ok(output)
}

pub fn get_size(url: &str) -> usize {
    let h = ureq::head(get_direct_from_booru(url).unwrap().as_ref()).call();
    let length = h.header("content-length").unwrap();

    length.parse().unwrap()
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

struct Selectors;

impl Selectors {
    pub fn no_match() -> Selector {
        Selector::parse(".nomatch").unwrap()
    }

    pub fn matches() -> Selector {
        Selector::parse("#pages > div").unwrap()
    }

    pub fn match_url() -> Selector {
        Selector::parse("table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > a:nth-child(1)").unwrap()
    }

    pub fn match_post_data() -> Selector {
        Selector::parse(
            "table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(1)",
        )
        .unwrap()
    }

    pub fn match_post_similarity() -> Selector {
        Selector::parse(
            "table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(1)",
        )
        .unwrap()
    }
}
