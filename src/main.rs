#![deny(clippy::disallowed_methods)]

use scraper::{Html, Selector};
use std::sync::LazyLock;

static DIV_FINDER: LazyLock<Selector> =
    LazyLock::new(|| scraper::Selector::parse(r#"div[role="article"]"#).expect("This is valid"));
static P_FINDER: LazyLock<Selector> =
    LazyLock::new(|| scraper::Selector::parse("p").expect("This is valid"));

fn main() -> std::io::Result<()> {
    let id_input = input("Enter the work id or url: ")?.trim().to_string();
    let work_id = match &id_input.get(..5) {
        Some("https") | Some("http") => id_input
            .split("/")
            .nth(4)
            .expect("An ao3 link with https at the start should have enough components"),
        Some("archi") => id_input
            .split("/")
            .nth(2)
            .expect("An ao3 link should have enough components"),
        _ => &id_input,
    };
    // Trim off `view_full_work` if present
    let work_id = work_id.split_once("?").map_or(work_id, |pair| pair.0);
    let url = format!("https://archiveofourown.org/works/{work_id}?view_full_work=true");

    let lengths = chapter_lengths(url);

    match lengths.len() {
        0 => panic!("Failed to get chapters properly - this work can't be measured"),
        1 => println!("{lengths:?}"),
        // The "first" chapter is the full length of the work, the rest are individual chapters
        _ => println!("{:?} {}", &lengths[1..], lengths[0]),
    }
    Ok(())
}

fn input(prompt: &str) -> std::io::Result<String> {
    let mut output = String::new();
    println!("{prompt}");
    std::io::stdin().read_line(&mut output)?;
    Ok(output)
}

fn get_document_ureq(url: &str) -> Html {
    let html_body = ureq::get(url)
        .call()
        .expect("Couldn't call url")
        .body_mut()
        .read_to_string()
        .expect("Should be able to read html to string");
    Html::parse_document(&html_body)
}

fn get_document_curl(url: &str) -> Html {
    let html_body = String::from_utf8(
        std::process::Command::new("curl")
            .args(["--http2", url])
            .output()
            .expect("Curl command failed")
            .stdout,
    )
    .expect("Failed to read to string");

    Html::parse_document(&html_body)
}

fn chapter_lengths(url: String) -> Vec<usize> {
    let document = get_document_ureq(&url);
    let document_curl;

    let chapters = {
        let ureq_chapters = document.select(&DIV_FINDER);
        if ureq_chapters.clone().count() == 0 {
            document_curl = get_document_curl(&url);
            document_curl.select(&DIV_FINDER)
        } else {
            ureq_chapters
        }
    };

    chapters
        .map(|chapter_text| {
            let words = chapter_text
                .select(&P_FINDER)
                .map(|p| p.text().collect::<String>())
                .collect::<Vec<String>>()
                .join(" ");

            words.replace("—", " ").split_whitespace().count()
        })
        .collect()
}
