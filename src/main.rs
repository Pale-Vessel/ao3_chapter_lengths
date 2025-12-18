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
    work_id.pop(); // Trim newline
    let url = format!(r"https://archiveofourown.org/works/{work_id}/navigate");
    println!("{url}");
    println!("{:?}", chapter_lengths(url, to_print));
    Ok(())
}

fn chapter_lengths(url: String, to_print: bool) -> Vec<u32> {
    let list_finder = scraper::Selector::parse("ol").unwrap();

    let html_body = ureq::get(url).call().expect("Couldn't call url").body_mut().read_to_string().unwrap();    
    
    maybe_print!(to_print, "got body");
    
    let document = scraper::Html::parse_document(&html_body);

    maybe_print!(to_print, "parse document");

    let chapter_list = document
        .select(&list_finder)
        .next()
        .expect("All chapter indices should have this element");

    maybe_print!(to_print, "{chapter_list:?}");
    todo!()
}
