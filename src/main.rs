use reqwest::blocking::get;
use scraper::{Html, Selector};
use pretty::RcDoc;

// Function to retrieve HTML content from a given URL
fn retrieve_html(url: &str) -> String {
    let response = get(url).unwrap().text().unwrap();
    response
}

fn scrape_quanta_magazine() {
    // Retrieve the HTML content of Quanta Magazine's webpage
    let url = "https://www.quantamagazine.org/computer-science/";
    let response = retrieve_html(url);

    // Parse the HTML document
    let html_doc = Html::parse_document(&response);

    // Define CSS selectors for specific elements
    let article_tag_selector = Selector::parse(".card__kicker").unwrap();
    let article_title_selector = Selector::parse(".card__title").unwrap();
    let author_name_selector = Selector::parse(".byline__author").unwrap();

    // Extract article tags, titles, and author names
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

    println!("From Quanta Magazine:");
    for ((title, tag), author) in article_titles.iter().zip(article_tags.iter()).zip(author_names.iter()) {
        println!("Title: {}", title);
        println!("Tag: {}", tag);
        println!("Author: {}\n", author);
    }
}

fn scrape_wired() {
    // Retrieve the HTML content of Wired's webpage
    let url = "https://www.wired.com/category/science/";
    let response = retrieve_html(url);

    let html_doc = Html::parse_document(&response);
    let article_title_selector = Selector::parse(".SummaryItemHedBase-hiFYpQ").unwrap();
    let article_tag_selector = Selector::parse(".RubricName-fVtemz").unwrap();
    let author_name_selector = Selector::parse(".BylineName-kwmrLn").unwrap();

    let article_titles: Vec<String> = html_doc
        .select(&article_title_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    let article_tags: Vec<String> = html_doc
        .select(&article_tag_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    let author_names: Vec<String> = html_doc
        .select(&author_name_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    println!("Articles:");
    for ((title, tag), author) in article_titles.iter().zip(article_tags.iter()).zip(author_names.iter()) {
        println!("Title: {}", title);
        println!("Tag: {}", tag);
        println!("Author: {}\n", author);
    }
}

fn scrape_tds() {
    let url = "https://towardsdatascience.com/";
    let response = retrieve_html(url);
    let html_doc = Html::parse_document(&response);

    let selector = Selector::parse(".streamItem.streamItem--section.js-streamItem").unwrap();

    for item in html_doc.select(&selector) {
        println!("{}", item.html());
    }
}


fn main() {
    scrape_quanta_magazine();
    scrape_wired();
    scrape_tds();
}
