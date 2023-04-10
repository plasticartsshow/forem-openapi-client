# \ReactionsApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_reactions_post**](ReactionsApi.md#api_reactions_post) | **POST** /api/reactions | create reaction
[**api_reactions_toggle_post**](ReactionsApi.md#api_reactions_toggle_post) | **POST** /api/reactions/toggle | toggle reaction



## api_reactions_post

> api_reactions_post(category, reactable_id, reactable_type)
create reaction

This endpoint allows the client to create a reaction to a specified reactable (eg, Article, Comment, or User). For examples:         * \"Like\"ing an Article will create a new \"like\" Reaction from the user for that Articles         * \"Like\"ing that Article a second time will return the previous \"like\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**reactable_id** | **i32** |  | [required] |
**reactable_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_reactions_toggle_post

> api_reactions_toggle_post(category, reactable_id, reactable_type)
toggle reaction

This endpoint allows the client to toggle the user's reaction to a specified reactable (eg, Article, Comment, or User). For examples:         * \"Like\"ing an Article will create a new \"like\" Reaction from the user for that Articles         * \"Like\"ing that Article a second time will remove the \"like\" from the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**reactable_id** | **i32** |  | [required] |
**reactable_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

