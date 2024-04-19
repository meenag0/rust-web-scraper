use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_rt::System;
use actix_cors::Cors;
use std::env;
use actix_web_static_files::ResourceFiles;
use actix_files::Files;

#[derive(Serialize)]
struct Article {
    title: String,
    author: String,
}

#[derive(Serialize)]
struct ArticleResponse {
    title: String,
    author: String,
    
}

// Function to retrieve HTML content from a given URL
async fn retrieve_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

async fn scrape_quanta_magazine() -> (Vec<String>, Vec<String>) {
    let url = "https://www.quantamagazine.org/computer-science/";
    let response = retrieve_html(url).await.expect("Failed to retrieve HTML");

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
    let mut titles = Vec::new();
    let mut authors = Vec::new();

    for ((title, tag), author) in article_titles.iter().zip(article_tags.iter()).zip(author_names.iter()) {
        titles.push(title.clone());
        authors.push(author.clone());
    }

    (titles, authors)
}

async fn scrape_tds() -> (Vec<String>, Vec<String>) {
    let url = "https://towardsdatascience.com/";
    let response = retrieve_html(url).await.expect("Failed to retrieve HTML");
    let html_doc = Html::parse_document(&response);

    let div_selector = Selector::parse(".col.u-xs-size12of12.js-trackPostPresentation.u-paddingLeft12").unwrap();

    let mut titles = Vec::new();
    let mut authors = Vec::new();

    // Find all elements matching the div selector
    for div in html_doc.select(&div_selector) {
        // Extract the title
        let title_selector = Selector::parse("h3").unwrap();
        if let Some(title_element) = div.select(&title_selector).next() {
            let title = title_element.text().collect::<String>().trim().to_owned();
            titles.push(title);
        }

        // Extract the author
        let author_selector = Selector::parse(".postMetaInline-authorLockup a").unwrap();
        if let Some(author_element) = div.select(&author_selector).next() {
            let author = author_element.text().collect::<String>().trim().to_owned();
            authors.push(author);
        }
    }

    (titles, authors)

}


async fn scrape_wired() -> (Vec<String>, Vec<String>) {

    let url: &str = "https://www.wired.com/category/science/";
    let response = retrieve_html(url).await.expect("Failed to retrieve HTML");

    let html_doc = Html::parse_document(&response);
    let article_title_selector = Selector::parse(".SummaryItemHedBase-hiFYpQ").unwrap();
    let article_tag_selector = Selector::parse(".RubricName-fVtemz").unwrap();
    let author_name_selector = Selector::parse(".BylineName-kwmrLn").unwrap();

    let article_titles: Vec<String> = html_doc
        .select(&article_title_selector)
        .map(|element| element.text().collect::<String>())
        .take(5)
        .collect();

    let article_tags: Vec<String> = html_doc
        .select(&article_tag_selector)
        .map(|element| element.text().collect::<String>())
        .take(5)
        .collect();

    let author_names: Vec<String> = html_doc
        .select(&author_name_selector)
        .map(|element| element.text().collect::<String>())
        .take(5)
        .collect();

    println!("Articles:");
    for ((title, tag), author) in article_titles.iter().zip(article_tags.iter()).zip(author_names.iter()) {
        println!("Title: {}", title);
        println!("Author: {}\n", author);
    }
        let mut titles = Vec::new();
        let mut authors = Vec::new();

        for ((title, tag), author) in article_titles.iter().zip(article_tags.iter()).zip(author_names.iter()) {
            titles.push(title.clone());
            authors.push(author.clone());
        }

        (titles, authors)
}


async fn scrape_sa() -> (Vec<String>, Vec<String>) {
    let url = "https://www.scientificamerican.com/";
    let response = retrieve_html(url).await.expect("Failed to retrieve HTML");
    let html_doc = Html::parse_document(&response);
    let div_selector = Selector::parse("div.articleList-R10iq.root-fREBs").unwrap();

    let title_selector = Selector::parse("h2.articleTitle-mtY5p").unwrap();
    let author_selector = Selector::parse("p.authors-NCGt1").unwrap();

    // Find all elements matching the title selector
    let titles = html_doc.select(&title_selector).take(5);
    let authors = html_doc.select(&author_selector).take(5);

    let mut scraped_titles = Vec::new();
    let mut scraped_authors = Vec::new();
    
    // Process each article and collect title and author
    for (title, author) in titles.zip(authors) {
        let title_text = title.text().collect::<String>();
        let author_text = author.text().collect::<String>();

        println!("Title: {}", title_text);
        println!("Author: {}", author_text);
        println!();

        scraped_titles.push(title_text);
        scraped_authors.push(author_text);
    }

    (scraped_titles, scraped_authors)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Fetch the host and port from environment variables
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    // Start the HTTP server]
    HttpServer::new(move || {
        let frontend_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static");


        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600),
            )
            .service(web::resource("/api").route(web::get().to(get_articles_handler)))
            // Serve frontend files from the "/static" path
            .service(Files::new("/", "static").show_files_listing())
            // Catch-all route for serving the index.html for frontend routes
            .default_service(web::get().to(index_html_handler))
            })
    .bind(format!("{}:{}", host, port))? // Bind to the specified host and port
    .run()
    .await
}

async fn index_html_handler() -> impl Responder {
    // Read the index.html file and return it as a response
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

fn extract_and_format_articles(articles: &[Article]) -> Vec<String> {
    let mut formatted_articles: Vec<String> = Vec::new();
    for article in articles {
        let formatted_article = format!("{} - by {}", article.title, article.author);
        formatted_articles.push(formatted_article);
    }
    formatted_articles
}

async fn get_articles_handler() -> impl Responder {
    let quanta_data = scrape_quanta_magazine().await;
    let tds_data = scrape_tds().await;
    let wired_data = scrape_wired().await;
    let sa_data = scrape_sa().await;

    let articles: Vec<Article> = combine_data(quanta_data, tds_data, wired_data, sa_data);
    let formatted_articles = extract_and_format_articles(&articles);

    let article_responses: Vec<ArticleResponse> = formatted_articles.iter().map(|article| {
        let parts: Vec<&str> = article.split(" - by ").collect();
        ArticleResponse {
            title: parts[0].to_string(),
            author: parts[1].to_string(),
        }
    }).collect();

    HttpResponse::Ok().json(article_responses)
}


fn combine_data(
    quanta_data: (Vec<String>, Vec<String>),
    tds_data: (Vec<String>, Vec<String>),
    wired_data: (Vec<String>, Vec<String>),
    sa_data: (Vec<String>, Vec<String>),
) -> Vec<Article> {
    // Combine data from different sources into a single vector
    let mut articles = Vec::new();

    // Combine data from Quanta Magazine
    for (title, author) in quanta_data.0.into_iter().zip(quanta_data.1) {
        articles.push(Article { title, author });
    }

    // Combine data from Towards Data Science
    for (title, author) in tds_data.0.into_iter().zip(tds_data.1) {
        articles.push(Article { title, author });
    }

    // Combine data from Wired
    for (title, author) in wired_data.0.into_iter().zip(wired_data.1) {
        articles.push(Article { title, author });
    }

    // Combine data from Scientific American
    for (title, author) in sa_data.0.into_iter().zip(sa_data.1) {
        articles.push(Article { title, author });
    }

    articles
}