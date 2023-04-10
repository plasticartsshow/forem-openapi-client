/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ArticleFlareTag : Flare tag of the article



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArticleFlareTag {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Background color (hexadecimal)
    #[serde(rename = "bg_color_hex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bg_color_hex: Option<Option<String>>,
    /// Text color (hexadecimal)
    #[serde(rename = "text_color_hex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub text_color_hex: Option<Option<String>>,
}

impl ArticleFlareTag {
    /// Flare tag of the article
    pub fn new() -> ArticleFlareTag {
        ArticleFlareTag {
            name: None,
            bg_color_hex: None,
            text_color_hex: None,
        }
    }
}

