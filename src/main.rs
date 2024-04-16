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

    // Define the CSS selector for the elements containing the links
    let class_selector = Selector::parse(".card__content").unwrap();

    // Iterate over each element selected by the class selector
    for element in html_doc.select(&class_selector) {
        // Extract the inner HTML content of the current element
        let inner_html = element.inner_html();
        println!("Inner HTML of .card__content: {}", inner_html);

        // Define a selector for the <a> tags within each .card__content element
        let link_selector = Selector::parse("a").unwrap();

        // Iterate over each <a> tag within the current .card__content element
        for link in element.select(&link_selector) {
            // Extract the value of the href attribute from the <a> tag
            if let Some(href) = link.value().attr("href") {
                println!("Href attribute of <a> tag: {}", href);
            }
        }
    }
}
