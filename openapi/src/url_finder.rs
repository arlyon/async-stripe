use anyhow::{anyhow, Result};
use heck::SnakeCase;
use reqwest::blocking::Client;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug)]
pub struct UrlFinder {
    html: String,
}

impl UrlFinder {
    pub fn new() -> Result<Self> {
        let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
        let resp = client.get("https://stripe.com/docs/api").send()?;

        if resp.status().is_success() {
            let text = resp.text()?;
            if text.contains("title: 'Stripe API Reference'") {
                Ok(Self { html: text })
            } else {
                Err(anyhow!("stripe api returned unexpected document"))
            }
        } else {
            log::error!("{}", resp.text()?);
            Err(anyhow!("request to stripe api returned non-200 status code"))
        }
    }

    pub fn url_for_object(&self, object: &str) -> Option<String> {
        let object_name = object.replace('.', "_").to_snake_case();
        let doc_url = format!("/{}s/object", object_name);
        if self.html.contains(&doc_url) {
            Some(format!("https://stripe.com/docs/api{}", doc_url))
        } else {
            log::warn!("{} not in html", doc_url);
            None
        }
    }
}
