/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ArticleIndex : Representation of an article or post returned in a list



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArticleIndex {
    #[serde(rename = "type_of")]
    pub type_of: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "cover_image", deserialize_with = "Option::deserialize")]
    pub cover_image: Option<String>,
    #[serde(rename = "readable_publish_date")]
    pub readable_publish_date: String,
    #[serde(rename = "social_image")]
    pub social_image: String,
    #[serde(rename = "tag_list")]
    pub tag_list: Vec<String>,
    #[serde(rename = "tags")]
    pub tags: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "canonical_url")]
    pub canonical_url: String,
    #[serde(rename = "positive_reactions_count")]
    pub positive_reactions_count: i32,
    #[serde(rename = "public_reactions_count")]
    pub public_reactions_count: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "edited_at", deserialize_with = "Option::deserialize")]
    pub edited_at: Option<String>,
    #[serde(rename = "crossposted_at", deserialize_with = "Option::deserialize")]
    pub crossposted_at: Option<String>,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "last_comment_at")]
    pub last_comment_at: String,
    /// Crossposting or published date time
    #[serde(rename = "published_timestamp")]
    pub published_timestamp: String,
    /// Reading time, in minutes
    #[serde(rename = "reading_time_minutes")]
    pub reading_time_minutes: i32,
    #[serde(rename = "user")]
    pub user: Box<crate::models::SharedUser>,
    #[serde(rename = "flare_tag", skip_serializing_if = "Option::is_none")]
    pub flare_tag: Option<Box<crate::models::ArticleFlareTag>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<crate::models::SharedOrganization>>,
}

impl ArticleIndex {
    /// Representation of an article or post returned in a list
    pub fn new(type_of: String, id: i32, title: String, description: String, cover_image: Option<String>, readable_publish_date: String, social_image: String, tag_list: Vec<String>, tags: String, slug: String, path: String, url: String, canonical_url: String, positive_reactions_count: i32, public_reactions_count: i32, created_at: String, edited_at: Option<String>, crossposted_at: Option<String>, published_at: String, last_comment_at: String, published_timestamp: String, reading_time_minutes: i32, user: crate::models::SharedUser) -> ArticleIndex {
        ArticleIndex {
            type_of,
            id,
            title,
            description,
            cover_image,
            readable_publish_date,
            social_image,
            tag_list,
            tags,
            slug,
            path,
            url,
            canonical_url,
            positive_reactions_count,
            public_reactions_count,
            created_at,
            edited_at,
            crossposted_at,
            published_at,
            last_comment_at,
            published_timestamp,
            reading_time_minutes,
            user: Box::new(user),
            flare_tag: None,
            organization: None,
        }
    }
}


