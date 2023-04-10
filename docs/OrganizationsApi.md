# \OrganizationsApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_org_articles**](OrganizationsApi.md#get_org_articles) | **GET** /api/organizations/{username}/articles | Organization's Articles
[**get_org_users**](OrganizationsApi.md#get_org_users) | **GET** /api/organizations/{username}/users | Organization's users
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /api/organizations/{username} | An organization



## get_org_articles

> Vec<crate::models::ArticleIndex> get_org_articles(username, page, per_page)
Organization's Articles

This endpoint allows the client to retrieve a list of Articles belonging to the organization  It supports pagination, each page will contain `30` users by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]

### Return type

[**Vec<crate::models::ArticleIndex>**](ArticleIndex.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_users

> Vec<crate::models::User> get_org_users(username, page, per_page)
Organization's users

This endpoint allows the client to retrieve a list of users belonging to the organization  It supports pagination, each page will contain `30` users by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> Vec<crate::models::Organization> get_organization(username)
An organization

This endpoint allows the client to retrieve a single organization by their username

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Organization>**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

