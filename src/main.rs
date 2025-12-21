use scraper::{Html, Selector};
use std::{sync::LazyLock, thread, time::Duration};

static OL_FINDER: LazyLock<Selector> = LazyLock::new(|| scraper::Selector::parse("ol").unwrap());
static LI_FINDER: LazyLock<Selector> = LazyLock::new(|| scraper::Selector::parse("li").unwrap());
static A_FINDER: LazyLock<Selector> = LazyLock::new(|| scraper::Selector::parse("a").unwrap());
static DIV_FINDER: LazyLock<Selector> = LazyLock::new(|| scraper::Selector::parse("div").unwrap());
static P_FINDER: LazyLock<Selector> = LazyLock::new(|| scraper::Selector::parse("p").unwrap());

macro_rules! maybe_print {
    ($cond: expr, $($print: expr), *) => {
        if $cond {
            println!($($print),*)
        }
    };
}

fn main() -> std::io::Result<()> {
    let mut to_print = String::new();
    println!("Debug printing? y/n: ");
    std::io::stdin().read_line(&mut to_print)?;
    let to_print = to_print.trim() == "y";
    let mut work_id = String::new();
    println!("Enter the work id: ");
    std::io::stdin().read_line(&mut work_id)?;
    let work_id = work_id.trim().to_string();
    let url = format!(r"https://archiveofourown.org/works/{work_id}/navigate");
    maybe_print!(to_print, "{url}");
    let l = chapter_lengths(url, to_print);
    println!("{l:?} {}", l.iter().cloned().sum::<usize>());
    Ok(())
}

fn chapter_lengths(url: String, to_print: bool) -> Vec<usize> {
    let html_body = ureq::get(url)
        .call()
        .expect("Couldn't call url")
        .body_mut()
        .read_to_string()
        .unwrap();

    maybe_print!(to_print, "got body");

    let document = Html::parse_document(&html_body);

    maybe_print!(to_print, "parsed document");

    let chapter_list = document
        .select(&OL_FINDER)
        .next()
        .expect("All chapter indices should have this element");

    let mut chapters = Vec::new();

    for (mut chapter_index, chapter) in chapter_list.select(&LI_FINDER).enumerate() {
        chapter_index += 1;
        let relative = chapter
            .select(&A_FINDER)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .trim();

        maybe_print!(to_print, "about to count words for chapter {chapter_index}");

        let chapter_link = format!(r"https://archiveofourown.org{relative}");

        chapters.push(chapter_length(chapter_link, chapter_index, to_print));
        thread::sleep(Duration::from_millis(500));
    }

    chapters
}

fn chapter_length(chapter_link: String, chapter_num: usize, to_print: bool) -> usize {
    let chapter_body = ureq::get(chapter_link)
        .call()
        .expect("Couldn't call url")
        .body_mut()
        .read_to_string()
        .unwrap();

    maybe_print!(to_print, "got response from chapter {chapter_num}");

    let document = Html::parse_document(&chapter_body);

    maybe_print!(to_print, "parsed text of {chapter_num}");

    let chapter_text = document
        .select(&DIV_FINDER)
        .find(|div| div.attr("role") == Some("article"))
        .unwrap();

    let words = chapter_text
        .select(&P_FINDER)
        .map(|p| p.text().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    words.replace("—", " ").split_whitespace().count()
}
