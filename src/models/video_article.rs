/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// VideoArticle : Representation of an Article with video

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VideoArticle {
    #[serde(rename = "type_of", skip_serializing_if = "Option::is_none")]
    pub type_of: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "cloudinary_video_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudinary_video_url: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(
        rename = "video_duration_in_minutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_duration_in_minutes: Option<String>,
    #[serde(rename = "video_source_url", skip_serializing_if = "Option::is_none")]
    pub video_source_url: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::VideoArticleUser>>,
}

impl VideoArticle {
    /// Representation of an Article with video
    pub fn new() -> VideoArticle {
        VideoArticle {
            type_of: None,
            id: None,
            path: None,
            cloudinary_video_url: None,
            title: None,
            user_id: None,
            video_duration_in_minutes: None,
            video_source_url: None,
            user: None,
        }
    }
}
