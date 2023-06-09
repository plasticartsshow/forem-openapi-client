/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Organization : Representation of an Organization

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "type_of", skip_serializing_if = "Option::is_none")]
    pub type_of: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "twitter_username", skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(rename = "github_username", skip_serializing_if = "Option::is_none")]
    pub github_username: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "joined_at", skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(rename = "tech_stack", skip_serializing_if = "Option::is_none")]
    pub tech_stack: Option<String>,
    #[serde(
        rename = "tag_line",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_line: Option<Option<String>>,
    #[serde(
        rename = "story",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub story: Option<Option<String>>,
}

impl Organization {
    /// Representation of an Organization
    pub fn new() -> Organization {
        Organization {
            type_of: None,
            username: None,
            name: None,
            summary: None,
            twitter_username: None,
            github_username: None,
            url: None,
            location: None,
            joined_at: None,
            tech_stack: None,
            tag_line: None,
            story: None,
        }
    }
}
