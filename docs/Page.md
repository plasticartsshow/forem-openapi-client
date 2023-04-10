# Page

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Title of the page | 
**slug** | **String** | Used to link to this page in URLs, must be unique and URL-safe | 
**description** | **String** | For internal use, helps similar pages from one another | 
**body_markdown** | Option<**String**> | The text (in markdown) of the ad (required) | [optional]
**body_json** | Option<**String**> | For JSON pages, the JSON body | [optional]
**is_top_level_path** | Option<**bool**> | If true, the page is available at '/{slug}' instead of '/page/{slug}', use with caution | [optional]
**social_image** | Option<[**serde_json::Value**](.md)> |  | [optional]
**template** | **String** | Controls what kind of layout the page is rendered in | [default to Contained]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


