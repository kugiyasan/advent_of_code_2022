use chrono::{Datelike, Local};
use reqwest::{header, Client};
use scraper::{Html, Selector};
use tokio::runtime::Runtime;

pub fn submit(answer: &str, part2: bool) {
    let now = Local::now();
    let rt = Runtime::new().unwrap();

    rt.block_on(async { _submit(now.year(), now.day(), part2, answer).await })
}

async fn _submit(year: i32, day: u32, part2: bool, answer: &str) {
    let part = if part2 { "part2" } else { "part1" };
    println!("Submitting AoC {year} day {day} {part}: {answer:?}");

    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");

    let cookies = include_str!("../../cookies.txt").trim();

    let level = if part2 { "2" } else { "1" };
    let form = [("level", level), ("answer", &answer)];

    let response = client
        .post(url)
        .header(header::COOKIE, cookies)
        .form(&form)
        .send()
        .await;

    match response {
        Ok(res) => {
            let html = res.text().await.unwrap();
            let document = Html::parse_document(&html);
            let selector = Selector::parse("main > article > p").unwrap();
            let text = document.select(&selector).next().unwrap();
            println!("{}", text.inner_html());
        }
        Err(err) => println!("FAILED: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit() {
        submit("1", false);
    }
}
