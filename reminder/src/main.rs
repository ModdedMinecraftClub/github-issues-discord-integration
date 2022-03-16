use anyhow::Context;
use common::config::Config;
use common::discord_dtos::{send_webhook, WebhookEmbed, WebhookMsgDto};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config =
        Config::from_file_or_default("config.yml").context("Failed to load configuration.")?;

    /*
    let webhook = WebhookMsgDto{
        content: "This is a test webhook sent from Rust.".into(),
        embeds: vec![
            WebhookEmbed{
                title: "Issue action has taken place: {}".into(),
                description: "Issue URL: https://blablabla.com".into()
            }
        ]
    };
    send_webhook("https://discord.com/api/webhooks/913647282858106931/-Nwrp8nnvL0omX-_4ChJJJaYcjOEDsPjRFIlFWxfcQcEyP9l7m5zqAtDukKR5UD2xmUH?wait=true", &webhook).await.unwrap();*/

    Ok(())
}
