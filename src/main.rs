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

    // CSS selectors for specific elements
    let article_tag_selector = Selector::parse(".card__kicker a").unwrap();
    let article_title_selector = Selector::parse(".card__title").unwrap();
    let author_name_selector = Selector::parse(".byline__author").unwrap();

    // Extract data using the selectors
    let article_tag = html_doc.select(&article_tag_selector).next().unwrap().text().collect::<String>();
    let article_title = html_doc.select(&article_title_selector).next().unwrap().text().collect::<String>();
    let author_name = html_doc.select(&author_name_selector).next().unwrap().text().collect::<String>();

    // Print the extracted data
    println!("Article Tag: {}", article_tag);
    println!("Article Title: {}", article_title);
    println!("Author Name: {}", author_name);
}
