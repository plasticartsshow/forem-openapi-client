# \ReadinglistApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_readinglist**](ReadinglistApi.md#get_readinglist) | **GET** /api/readinglist | Readinglist



## get_readinglist

> Vec<crate::models::ArticleIndex> get_readinglist(page, per_page)
Readinglist

This endpoint allows the client to retrieve a list of articles that were saved to a Users readinglist.         It supports pagination, each page will contain `30` articles by default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]

### Return type

[**Vec<crate::models::ArticleIndex>**](ArticleIndex.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

