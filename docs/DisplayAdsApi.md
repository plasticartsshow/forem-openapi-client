# \DisplayAdsApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_display_ads_get**](DisplayAdsApi.md#api_display_ads_get) | **GET** /api/display_ads | display ads
[**api_display_ads_id_get**](DisplayAdsApi.md#api_display_ads_id_get) | **GET** /api/display_ads/{id} | display ad
[**api_display_ads_id_put**](DisplayAdsApi.md#api_display_ads_id_put) | **PUT** /api/display_ads/{id} | display ads
[**api_display_ads_id_unpublish_put**](DisplayAdsApi.md#api_display_ads_id_unpublish_put) | **PUT** /api/display_ads/{id}/unpublish | unpublish
[**api_display_ads_post**](DisplayAdsApi.md#api_display_ads_post) | **POST** /api/display_ads | display ads



## api_display_ads_get

> Vec<crate::models::DisplayAd> api_display_ads_get()
display ads

This endpoint allows the client to retrieve a list of all display ads.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DisplayAd>**](DisplayAd.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_display_ads_id_get

> api_display_ads_id_get(id)
display ad

This endpoint allows the client to retrieve a single display ad, via its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the display ad. | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_display_ads_id_put

> Vec<crate::models::DisplayAd> api_display_ads_id_put(id, display_ad)
display ads

This endpoint allows the client to update the attributes of a single display ad, via its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the display ad. | [required] |
**display_ad** | Option<[**Vec<crate::models::DisplayAd>**](DisplayAd.md)> |  |  |

### Return type

[**Vec<crate::models::DisplayAd>**](DisplayAd.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_display_ads_id_unpublish_put

> api_display_ads_id_unpublish_put(id)
unpublish

This endpoint allows the client to remove a display ad from rotation by un-publishing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the display ad to unpublish. | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_display_ads_post

> Vec<crate::models::DisplayAd> api_display_ads_post(display_ad)
display ads

This endpoint allows the client to create a new display ad.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_ad** | Option<[**Vec<crate::models::DisplayAd>**](DisplayAd.md)> |  |  |

### Return type

[**Vec<crate::models::DisplayAd>**](DisplayAd.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

