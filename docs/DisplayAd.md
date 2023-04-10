# DisplayAd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the Display Ad | [optional]
**name** | **String** | For internal use, helps distinguish ads from one another | 
**body_markdown** | **String** | The text (in markdown) of the ad (required) | 
**approved** | Option<**bool**> | Ad must be both published and approved to be in rotation | [optional]
**published** | Option<**bool**> | Ad must be both published and approved to be in rotation | [optional]
**organization_id** | Option<**i32**> | Identifies the organization to which the ad belongs | [optional]
**creator_id** | Option<**i32**> | Identifies the user who created the ad. | [optional]
**placement_area** | **String** | Identifies which area of site layout the ad can appear in | 
**tag_list** | Option<**String**> | Tags on which this ad can be displayed (blank is all/any tags) | [optional]
**article_exclude_ids** | Option<**String**> | Articles this ad should *not* appear on (blank means no articles are disallowed, and this ad can appear next to any/all articles). Comma-separated list of integer Article IDs | [optional]
**display_to** | Option<**String**> | Potentially limits visitors to whom the ad is visible | [optional][default to All]
**type_of** | Option<**String**> | Types of the billboards: in_house (created by admins), community (created by an entity, appears on entity's content), external ( created by an entity, or a non-entity, can appear everywhere)  | [optional][default to InHouse]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


