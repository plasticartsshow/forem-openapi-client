# \TagsApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_followed_tags**](TagsApi.md#get_followed_tags) | **GET** /api/follows/tags | Followed Tags
[**get_tags**](TagsApi.md#get_tags) | **GET** /api/tags | Tags



## get_followed_tags

> Vec<crate::models::FollowedTag> get_followed_tags()
Followed Tags

This endpoint allows the client to retrieve a list of the tags they follow.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::FollowedTag>**](FollowedTag.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> Vec<crate::models::Tag> get_tags(page, per_page)
Tags

This endpoint allows the client to retrieve a list of tags that can be used to tag articles.  It will return tags ordered by popularity.  It supports pagination, each page will contain 10 tags by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 10]

### Return type

[**Vec<crate::models::Tag>**](Tag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

