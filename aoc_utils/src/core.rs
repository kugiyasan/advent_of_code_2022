use reqwest::{header, Client};
use pyo3::prelude::*;

pub async fn _submit(year: i32, day: u32, part2: bool, answer: String) -> PyResult<()> {
    println!("{}", answer);

    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");

    let cookies = include_str!("../../cookies.txt");

    let level = if part2 { "2" } else { "1" };
    let form = [("level", level), ("answer", &answer)];

    let res = client
        .post(url)
        .header(header::COOKIE, cookies)
        .form(&form)
        .send()
        .await;

    // match res {
    //     Ok(response) => {
    //         if response.status().is_success() {
    //             Ok(())
    //         } else {
    //             // Err(PyErr::new_err(format!(
    //             //     "Failed to submit answer: {}",
    //             //     response.text().await.unwrap_or_else(|_| String::from(""))
    //             // )))
    //         }
    //     }
    //     Err(_) => Err(PyErr::new_err(format!(
    //         "Failed to submit answer: {}",
    //         answer
    //     ))),
    // }

    println!("{:?}", res);

    Ok(())
}
