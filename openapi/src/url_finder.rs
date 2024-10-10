use std::collections::HashMap;

use anyhow::{anyhow, Result};
use heck::SnakeCase;
use reqwest::blocking::Client;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug)]
pub struct UrlFinder {
    url_lookup: HashMap<String, String>,
}

impl UrlFinder {
    pub fn new() -> Result<Self> {
        let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
        let resp = client.get("https://stripe.com/docs/api").send()?;

        if resp.status().is_success() {
            let text = resp.text()?;
            if let Some(line) = text.lines().find(|l| l.contains("window.__INITIAL_STATE__ = ")) {
                let initial_state: StripeInitialState = serde_json::from_str(
                    line.trim()
                        .trim_start_matches("window.__INITIAL_STATE__ = ")
                        .trim_end_matches(';'),
                )
                .expect("should be valid json");
                Ok(Self { url_lookup: initial_state.into() })
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
        tracing::debug!("looking for {} in html", object_name);
        let object_names = [format!("{}_object", object_name), object_name];
        for name in object_names {
            if let Some(path) = self.url_lookup.get(&name) {
                return Some(format!("https://stripe.com/docs{}", path));
            }
        }

        tracing::warn!("{} not in html", object);
        None
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StripeInitialState {
    article: Article,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Article {
    lazily_loaded_content: LazilyLoadedContent,
}

impl From<StripeInitialState> for HashMap<String, String> {
    fn from(value: StripeInitialState) -> Self {
        value
            .article
            .lazily_loaded_content
            .after
            .into_iter()
            .flat_map(|after| {
                let routes = after.attributes.section_routes;
                let anchors = after.attributes.anchors;
                assert!(anchors.len() == routes.len());

                // anchors have been rotated -1 so we need to cycle sectionRoutes to match
                anchors.into_iter().zip(routes.into_iter().cycle().skip(1))
            })
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LazilyLoadedContent {
    after: Vec<AfterItem>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AfterItem {
    attributes: Attributes,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    #[serde(default)]
    anchors: Vec<String>,
    #[serde(default)]
    section_routes: Vec<String>,
}
