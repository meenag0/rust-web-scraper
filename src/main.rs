use reqwest::blocking::get;
use scraper::{Html, Selector};

fn retrieve_html() -> String {
    let response = get("https://www.quantamagazine.org/computer-science/").unwrap().text().unwrap();
    return response;
}

fn main() {
    // Retrieve the HTML content of the webpage
    let response = retrieve_html();

    // Parse the HTML document
    let html_doc = Html::parse_document(&response);

    // CSS selectors for elements
    let article_tag_selector = Selector::parse(".card__kicker a").unwrap();
    let article_title_selector = Selector::parse(".card__title").unwrap();
    let author_name_selector = Selector::parse(".byline__author").unwrap();

    // Extract data using the selectors
    let article_tags: Vec<String> = html_doc
        .select(&article_tag_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    let article_titles: Vec<String> = html_doc
        .select(&article_title_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    let author_names: Vec<String> = html_doc
        .select(&author_name_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    // Print the extracted data
    println!("Article Tags: {:?}", article_tags);
    println!("Article Titles: {:?}", article_titles);
    println!("Author Names: {:?}", author_names);
}
