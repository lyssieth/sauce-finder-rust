use crate::iqdb::Matches;

fn check_url(url: &str) -> bool {
    let head = ureq::head(url);

    head.header("content-type").unwrap_or("").contains("image")
}

fn build_match(url: &str) -> Matches {
    Matches::default()
}
