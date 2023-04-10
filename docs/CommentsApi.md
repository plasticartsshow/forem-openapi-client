# \CommentsApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_comment_by_id**](CommentsApi.md#get_comment_by_id) | **GET** /api/comments/{id} | Comment by id
[**get_comments_by_article_id**](CommentsApi.md#get_comments_by_article_id) | **GET** /api/comments | Comments



## get_comment_by_id

> get_comment_by_id(id)
Comment by id

This endpoint allows the client to retrieve a comment as well as his descendants comments.    It will return the required comment (the root) with its nested descendants as a thread.    See the format specification for further details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Comment identifier. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_by_article_id

> Vec<crate::models::Comment> get_comments_by_article_id(a_id, p_id)
Comments

This endpoint allows the client to retrieve all comments belonging to an article or podcast episode as threaded conversations.  It will return the all top level comments with their nested comments as threads. See the format specification for further details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a_id** | Option<**String**> | Article identifier. |  |
**p_id** | Option<**String**> | Podcast Episode identifier. |  |

### Return type

[**Vec<crate::models::Comment>**](Comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

