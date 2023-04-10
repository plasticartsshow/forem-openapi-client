/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProfileImage : A profile image object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProfileImage {
    /// Return profile_image
    #[serde(rename = "type_of", skip_serializing_if = "Option::is_none")]
    pub type_of: Option<String>,
    /// Determines the type of the profile image owner (user or organization)
    #[serde(rename = "image_of", skip_serializing_if = "Option::is_none")]
    pub image_of: Option<String>,
    /// Profile image (640x640)
    #[serde(rename = "profile_image", skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    /// Profile image (90x90)
    #[serde(rename = "profile_image_90", skip_serializing_if = "Option::is_none")]
    pub profile_image_90: Option<String>,
}

impl ProfileImage {
    /// A profile image object
    pub fn new() -> ProfileImage {
        ProfileImage {
            type_of: None,
            image_of: None,
            profile_image: None,
            profile_image_90: None,
        }
    }
}
