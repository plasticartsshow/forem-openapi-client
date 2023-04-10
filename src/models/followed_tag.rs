/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FollowedTag : Representation of a followed tag

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FollowedTag {
    /// Tag id
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "points")]
    pub points: f32,
}

impl FollowedTag {
    /// Representation of a followed tag
    pub fn new(id: i64, name: String, points: f32) -> FollowedTag {
        FollowedTag { id, name, points }
    }
}
