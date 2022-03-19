use anyhow::Context;
use common::config::Config;
use common::discord_dtos::{
    build_ping_list_from_github_dto, send_webhook, WebhookEmbed, WebhookMsgDto,
};
use common::github_dtos::IssueDto;
use reqwest::Client;

async fn fetch_issues(config: &Config) -> anyhow::Result<Vec<IssueDto>> {
    let url = format!(
        "https://api.github.com/repos/{}/issues",
        config.github_repo_identifier
    );
    log::info!("Fetching from URL: {}", url);

    let response = Client::new()
        .get(url)
        .header(
            "User-Agent",
            "Github Issues Discord Integration MMCC (john01dav@gmail.com)",
        )
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let config =
        Config::from_file_or_default("config.yml").context("Failed to load configuration.")?;

    let issues = fetch_issues(&config)
        .await
        .context("Failed to fetch issues from Github")?;

    for issue in issues {
        log::info!("Reminding for issue: {}", issue.title);

        let mut label_list = String::new();
        for tag in issue.labels.iter().take(1) {
            label_list.push_str(&tag.name);
        }
        for tag in issue.labels.iter().skip(1) {
            label_list.push_str(", ");
            label_list.push_str(&tag.name);
        }

        let webhook = WebhookMsgDto {
            content: build_ping_list_from_github_dto(
                &config,
                issue.labels.iter().map(|label| label.name.as_str()),
            ),
            embeds: vec![WebhookEmbed {
                description: format!(
                    r#"**Friendly reminder that the [issue "{}"]({}) is currently open and assigned to you :slight_smile:.**
Labels: {}"#,
                    issue.title, issue.html_url, label_list
                ),
                title: format!("Issue Reminder",),
                color: if issue.labels.is_empty() {
                    0
                } else {
                    i64::from_str_radix(&issue.labels[0].color, 16).unwrap_or(0)
                },
            }],
        };
        send_webhook(&config.webhook_url, &webhook).await.unwrap();
    }

    Ok(())
}
