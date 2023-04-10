# \ProfileImagesApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile_image**](ProfileImagesApi.md#get_profile_image) | **GET** /api/profile_images/{username} | A Users or organizations profile image



## get_profile_image

> Vec<crate::models::ProfileImage> get_profile_image(username)
A Users or organizations profile image

This endpoint allows the client to retrieve a user or organization profile image information by its         corresponding username.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The parameter is the username of the user or the username of the organization. | [required] |

### Return type

[**Vec<crate::models::ProfileImage>**](ProfileImage.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

