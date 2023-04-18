use anyhow::{anyhow, Result};
use heck::SnakeCase;
use reqwest::blocking::Client;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug)]
pub struct UrlFinder {
    flattened_api_sections: serde_json::Map<String, serde_json::Value>,
}

impl UrlFinder {
    pub fn new() -> Result<Self> {
        let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
        let resp = client.get("https://stripe.com/docs/api").send()?;

        if resp.status().is_success() {
            let text = resp.text()?;
            if let Some(line) = text.lines().find(|l| l.contains("flattenedAPISections: {")) {
                Ok(Self {
                    flattened_api_sections: serde_json::from_str(
                        line.trim()
                            .trim_start_matches("flattenedAPISections: ")
                            .trim_end_matches(","),
                    )
                    .expect("should be valid json"),
                })
            } else {
                Err(anyhow!("stripe api returned unexpected document"))
            }
        } else {
            tracing::error!("{}", resp.text()?);
            Err(anyhow!("request to stripe api returned non-200 status code"))
        }
    }

    pub fn url_for_object(&self, object: &str) -> Option<String> {
        let object_name = object.replace('.', "_").to_snake_case();
        let object_names = [format!("{}_object", object_name), object_name];
        for name in object_names {
            if let Some(path) = self
                .flattened_api_sections
                .get(&name)
                .and_then(|o| o.as_object().expect("this should be an object").get("path"))
                .and_then(|s| s.as_str())
            {
                return Some(format!("https://stripe.com/docs/api{}", path));
            }
        }

        tracing::warn!("{} not in html", object);
        None
    }
}
