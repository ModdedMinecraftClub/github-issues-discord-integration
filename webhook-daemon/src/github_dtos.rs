use common::github_dtos::IssueDto;
use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct WebhookPayloadDto {
    pub action: String,
    pub issue: IssueDto,
}

#[test]
fn test_dto_decode() {
    let dto: WebhookPayloadDto = serde_json::from_str(TEST_WEBHOOK_PAYLOAD).unwrap();
    let expected = WebhookPayloadDto {
        action: "created".into(),
        issue: IssueDto {
            html_url: "https://github.com/Codertocat/Hello-World/issues/1".into(),
            title: "Spelling error in the README file".into(),
            labels: vec![Label {
                name: "bug".into(),
                color: "d73a4a".into(),
            }],
        },
    };
    assert_eq!(expected, dto);
}
