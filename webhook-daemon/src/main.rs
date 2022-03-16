use crate::github_dtos::WebhookPayloadDto;
use actix_web::web::{Data, Json};
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use common::config::Config;
use common::discord_dtos::{
    build_ping_list_from_github_dto, send_webhook, WebhookEmbed, WebhookMsgDto,
};
use std::collections::BTreeSet;
use std::ops::Deref;

mod github_dtos;

#[post("/")]
async fn handle_github_webhook(
    config: Data<Config>,
    dto: Json<WebhookPayloadDto>,
) -> impl Responder {
    log::info!(
        r#"Got webhook from Github for issue "{}" with action "{}" at URL "{}"."#,
        dto.issue.title,
        dto.action,
        dto.issue.html_url
    );

    match try_handle_github_webhook(config.get_ref(), dto.deref()).await {
        Ok(_) => HttpResponse::Ok().await,
        Err(err) => {
            log::error!("Failed to process Github webhook: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}

async fn try_handle_github_webhook(config: &Config, dto: &WebhookPayloadDto) -> anyhow::Result<()> {
    let mut label_list = String::new();
    for tag in dto.issue.labels.iter().take(1) {
        label_list.push_str(&tag.name);
    }
    for tag in dto.issue.labels.iter().skip(1) {
        label_list.push_str(", ");
        label_list.push_str(&tag.name);
    }

    let discord_message = WebhookMsgDto {
        content: build_ping_list_from_github_dto(
            config,
            dto.issue.labels.iter().map(|label| label.name.as_str()),
        ),
        embeds: vec![WebhookEmbed {
            description: format!(
                r#"**The [issue "{}"]({}) has been {}.**
Labels: {}"#,
                dto.issue.title, dto.issue.html_url, dto.action, label_list
            ),
            title: format!("Issue {}!", dto.action),
            color: if dto.issue.labels.is_empty() {
                0
            } else {
                i64::from_str_radix(&dto.issue.labels[0].color, 16).unwrap_or(0)
            },
        }],
    };
    send_webhook(&config.webhook_url, &discord_message)
        .await
        .context("Failed to send webhook to Discord")?;

    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let config =
        Config::from_file_or_default("config.yml").context("Failed to load configuration.")?;

    log::info!("Binding to address: {}", config.bind_address);
    let bind_address = config.bind_address.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .service(handle_github_webhook)
    })
    .bind(&bind_address)?
    .run()
    .await?;

    Ok(())
}
