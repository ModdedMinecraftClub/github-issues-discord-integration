use crate::config::Config;
use anyhow::Context;
use reqwest::Client;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Serialize)]
pub struct WebhookMsgDto {
    pub content: String,
    pub embeds: Vec<WebhookEmbed>,
}

#[derive(Serialize)]
pub struct WebhookEmbed {
    pub title: String,
    pub color: i64,
    pub description: String,
}

pub async fn send_webhook(url: &str, webhook: &WebhookMsgDto) -> anyhow::Result<()> {
    let client = Client::new();
    client
        .post(url)
        .json(webhook)
        .send()
        .await
        .with_context(|| format!("Failed to send webhook to URL: {}", url))?;
    Ok(())
}

pub fn build_ping_list_from_github_dto<'a>(
    config: &'a Config,
    labels: impl Iterator<Item = &'a str>,
) -> String {
    let mut people_to_ping = BTreeSet::new();
    for label in labels {
        if let Some(pingees) = config.labels.get(label) {
            for pingee in pingees {
                people_to_ping.insert(pingee);
            }
        }
    }

    if people_to_ping.is_empty() {
        for pingee in &config.unlabeled {
            people_to_ping.insert(pingee);
        }
    }

    let mut ping_list = String::new();
    for person_to_ping in people_to_ping {
        ping_list.push_str(&format!("<{}>", person_to_ping));
    }

    ping_list
}
