/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Comment : A Comment on an Article or Podcast Episode



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "type_of", skip_serializing_if = "Option::is_none")]
    pub type_of: Option<String>,
    #[serde(rename = "id_code", skip_serializing_if = "Option::is_none")]
    pub id_code: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Podcast image url
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl Comment {
    /// A Comment on an Article or Podcast Episode
    pub fn new() -> Comment {
        Comment {
            type_of: None,
            id_code: None,
            created_at: None,
            image_url: None,
        }
    }
}

