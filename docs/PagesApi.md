# \PagesApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_pages_get**](PagesApi.md#api_pages_get) | **GET** /api/pages | show details for all pages
[**api_pages_id_delete**](PagesApi.md#api_pages_id_delete) | **DELETE** /api/pages/{id} | remove a page
[**api_pages_id_get**](PagesApi.md#api_pages_id_get) | **GET** /api/pages/{id} | show details for a page
[**api_pages_id_put**](PagesApi.md#api_pages_id_put) | **PUT** /api/pages/{id} | update details for a page
[**api_pages_post**](PagesApi.md#api_pages_post) | **POST** /api/pages | pages



## api_pages_get

> Vec<crate::models::Page> api_pages_get()
show details for all pages

This endpoint allows the client to retrieve details for all Page objects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Page>**](Page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_pages_id_delete

> crate::models::Page api_pages_id_delete(id)
remove a page

This endpoint allows the client to delete a single Page object, specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the page. | [required] |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_pages_id_get

> crate::models::Page api_pages_id_get(id)
show details for a page

This endpoint allows the client to retrieve details for a single Page object, specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the page. | [required] |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_pages_id_put

> crate::models::Page api_pages_id_put(id, page)
update details for a page

This endpoint allows the client to retrieve details for a single Page object, specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the page. | [required] |
**page** | Option<[**Page**](Page.md)> |  |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_pages_post

> api_pages_post(api_pages_post_request)
pages

This endpoint allows the client to create a new page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_pages_post_request** | Option<[**ApiPagesPostRequest**](ApiPagesPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

