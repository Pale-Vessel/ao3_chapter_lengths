fn main() -> std::io::Result<()> {
    let mut work_id = String::new();
    println!("Enter the work id: ");
    std::io::stdin().read_line(&mut work_id)?;
    work_id.pop(); // Trim newline
    let url = format!(r"https://archiveofourown.org/works/{work_id}/navigate");
    println!("{url}");
    println!("{:?}", chapter_lengths(url));
    Ok(())
}

fn chapter_lengths(url: String) -> Vec<u32> {
    let list_finder = scraper::Selector::parse("ol").unwrap();

    let html_body = ureq::get(url).call().expect("Couldn't call url").body_mut().read_to_string().unwrap();    
    println!("got body");
    
    let document = scraper::Html::parse_document(&html_body);

    let chapter_list = document
        .select(&list_finder)
        .next()
        .expect("All chapter indices should have this element");

    println!("{chapter_list:?}");
    todo!()
}
