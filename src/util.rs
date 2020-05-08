use serde_json::json;
use ureq;

fn get_iqdb(url: &str) -> ureq::Response {
    let data = json!({
        "file": "(binary)",
        "url": url
    });
    ureq::post(url).send_json(data)
}

fn check_url(url: &str) -> bool {
    let head = ureq::head(url);

    head.header("content-type").unwrap_or("").contains("image")
}
