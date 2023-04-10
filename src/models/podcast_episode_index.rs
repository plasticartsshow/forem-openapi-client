/*
 * Forem API V1
 *
 * Access Forem articles, users and other resources via API.         For a real-world example of Forem in action, check out [DEV](https://www.dev.to).         All endpoints can be accessed with the 'api-key' header and a accept header, but         some of them are accessible publicly without authentication.          Dates and date times, unless otherwise specified, must be in         the [RFC 3339](https://tools.ietf.org/html/rfc3339) format.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PodcastEpisodeIndex : Representation of a podcast episode returned in a list



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PodcastEpisodeIndex {
    #[serde(rename = "type_of")]
    pub type_of: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "class_name")]
    pub class_name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "title")]
    pub title: String,
    /// Podcast episode image url or podcast image url
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "podcast")]
    pub podcast: Box<crate::models::SharedPodcast>,
}

impl PodcastEpisodeIndex {
    /// Representation of a podcast episode returned in a list
    pub fn new(type_of: String, id: i32, class_name: String, path: String, title: String, image_url: String, podcast: crate::models::SharedPodcast) -> PodcastEpisodeIndex {
        PodcastEpisodeIndex {
            type_of,
            id,
            class_name,
            path,
            title,
            image_url,
            podcast: Box::new(podcast),
        }
    }
}


