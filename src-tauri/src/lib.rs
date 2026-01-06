use serde::Serialize;
use scraper::{Html, Selector};
use reqwest::Client;
use url::Url;

#[derive(Serialize)] 
pub struct LinkResult {
    pub url: String,
    pub status: u16,
}

pub async fn check_page_links_impl(url: String) -> Result<Vec<LinkResult>, String> {
    let client = Client::new();
    
    //  Fetch the main page HTML
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect: {}", e))?;
    
    let html_text = response.text().await.map_err(|e| e.to_string())?;

    //  Extract links in a scope where 'Selector' doesn't cross an await point
    let base_url = Url::parse(&url).map_err(|e| e.to_string())?;
    let mut links_to_check = Vec::new();

    {
        // Everything in this { } block is dropped before we await again
        let document = Html::parse_document(&html_text);
        let selector = Selector::parse("a").map_err(|_| "Invalid selector")?;
        
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(full_url) = base_url.join(href) {
                    if full_url.scheme().starts_with("http") {
                        links_to_check.push(full_url.to_string());
                    }
                }
            }
        }
    } 

    //  Now check the statuses
    let mut results = Vec::new();
    for link in links_to_check {
        // Use .send().await without the ? operator here 
        // so that one bad link doesn't crash the whole scan.
        let status = match client.get(&link).send().await {
            Ok(resp) => resp.status().as_u16(),
            Err(_) => 404, // If we can't connect, treat it as a broken link
        };

        results.push(LinkResult {
            url: link,
            status,
        });
    }

    Ok(results)
}