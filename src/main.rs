#![deny(clippy::disallowed_methods)]

use scraper::{Html, Selector};
use std::sync::LazyLock;

static DIV_FINDER: LazyLock<Selector> =
    LazyLock::new(|| scraper::Selector::parse(r#"div[role="article"]"#).expect("This is valid"));
static P_FINDER: LazyLock<Selector> =
    LazyLock::new(|| scraper::Selector::parse("p").expect("This is valid"));

macro_rules! maybe_print {
    ($cond: expr, $($print: expr), *) => {
        if $cond {
            println!($($print),*)
        }
    };
}

fn main() -> std::io::Result<()> {
    let to_print = input("Debug printing? y/n: ")?;
    let to_print = to_print.trim() == "y";

    let id_input = input("Enter the work id or url: ")?.trim().to_string();
    let work_id = match &id_input.get(..5) {
        Some("https") => id_input
            .split("/")
            .nth(4)
            .expect("An ao3 link with https at the start should have enough components"),
        Some("archi") => id_input
            .split("/")
            .nth(2)
            .expect("An ao3 link should have enough components"),
        _ => &id_input,
    };
    let url = format!(r"https://archiveofourown.org/works/{work_id}?view_full_work=true");

    maybe_print!(to_print, "{url}");

    let lengths = chapter_lengths(url, to_print);

    match lengths.len() {
        0 => panic!("Failed to get chapters properly - this work can't be measured"),
        1 => println!("{lengths:?}"),
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

fn chapter_lengths(url: String, to_print: bool) -> Vec<usize> {
    let html_body = ureq::get(url)
        .call()
        .expect("Couldn't call url")
        .body_mut()
        .read_to_string()
        .expect("Should be able to read html to string");

    maybe_print!(to_print, "got body");

    let document = Html::parse_document(&html_body);

    maybe_print!(to_print, "parsed document");

    let chapters = document.select(&DIV_FINDER);

    let k = chapters.clone();
    println!("{:?}", k.collect::<Vec<_>>());

    chapters
        //.skip(1)
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
