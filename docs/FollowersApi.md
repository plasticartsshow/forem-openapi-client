# \FollowersApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_followers**](FollowersApi.md#get_followers) | **GET** /api/followers/users | Followers



## get_followers

> Vec<crate::models::GetFollowers200ResponseInner> get_followers(page, per_page, sort)
Followers

This endpoint allows the client to retrieve a list of the followers they have.         \"Followers\" are users that are following other users on the website.         It supports pagination, each page will contain 80 followers by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]
**sort** | Option<**String**> | Default is 'created_at'. Specifies the sort order for the created_at param of the follow                                 relationship. To sort by newest followers first (descending order) specify                                 ?sort=-created_at. |  |

### Return type

[**Vec<crate::models::GetFollowers200ResponseInner>**](getFollowers_200_response_inner.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

