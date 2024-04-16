use reqwest::blocking::get;
use scraper::{Html, Selector};
use pretty::RcDoc;
use std::error::Error;

// Function to retrieve HTML content from a given URL
fn retrieve_html(url: &str) -> String {
    let response = get(url).unwrap().text().unwrap();
    response
}

fn scrape_quanta_magazine() {
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
        println!("Author: {}\n", author);
    }
}

fn scrape_wired() {

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
        println!("Author: {}\n", author);
    }
}

fn scrape_tds() {
    let url = "https://towardsdatascience.com/";
    let response = retrieve_html(url);
    let html_doc = Html::parse_document(&response);

    let div_selector = Selector::parse(".col.u-xs-size12of12.js-trackPostPresentation.u-paddingLeft12").unwrap();

    // Find all elements matching the div selector
    for div in html_doc.select(&div_selector) {
        // Extract the title
        let title_selector = Selector::parse("h3").unwrap();
        let title_element = div.select(&title_selector).next().unwrap();
        let title = title_element.text().collect::<String>().trim().to_owned();

        // Extract the author
        let author_selector = Selector::parse(".postMetaInline-authorLockup a").unwrap();
        let author_element = div.select(&author_selector).next().unwrap();
        let author = author_element.text().collect::<String>().trim().to_owned();

        println!("Title: {}", title);
        println!("Author: {}", author);
        println!();
    }
}


fn scrape_sa() {
    let url = "https://www.scientificamerican.com/";
    let response = retrieve_html(url);
    let html_doc = Html::parse_document(&response);
    let div_selector = Selector::parse("div.articleList-R10iq.root-fREBs").unwrap();

    // let div_elements = html_doc.select(&div_selector);

    // for div_element in div_elements {
    //     // Extract the inner HTML content of the <div> element
    //     let inner_html = div_element.inner_html();

    //     println!("{}", inner_html);
    // }

    let title_selector = Selector::parse("h2.articleTitle-mtY5p").unwrap();
    let author_selector = Selector::parse("p.authors-NCGt1").unwrap();

    // Find all elements matching the title selector
    let titles = html_doc.select(&title_selector);
    let authors = html_doc.select(&author_selector);

    // Process each article and print title and author
    for (title, author) in titles.zip(authors) {
        let title_text = title.text().collect::<String>();
        let author_text = author.text().collect::<String>();

        println!("Title: {}", title_text);
        println!("Author: {}", author_text);
        println!();
    }
}


fn main() {
    scrape_quanta_magazine();
    scrape_wired();
    scrape_tds();
    scrape_sa();
}
