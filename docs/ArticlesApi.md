# \ArticlesApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_article**](ArticlesApi.md#create_article) | **POST** /api/articles | Publish article
[**get_article_by_id**](ArticlesApi.md#get_article_by_id) | **GET** /api/articles/{id} | Published article by id
[**get_article_by_path**](ArticlesApi.md#get_article_by_path) | **GET** /api/articles/{username}/{slug} | Published article by path
[**get_articles**](ArticlesApi.md#get_articles) | **GET** /api/articles | Published articles
[**get_latest_articles**](ArticlesApi.md#get_latest_articles) | **GET** /api/articles/latest | Published articles sorted by published date
[**get_org_articles**](ArticlesApi.md#get_org_articles) | **GET** /api/organizations/{username}/articles | Organization's Articles
[**get_user_all_articles**](ArticlesApi.md#get_user_all_articles) | **GET** /api/articles/me/all | User's all articles
[**get_user_articles**](ArticlesApi.md#get_user_articles) | **GET** /api/articles/me | User's articles
[**get_user_published_articles**](ArticlesApi.md#get_user_published_articles) | **GET** /api/articles/me/published | User's published articles
[**get_user_unpublished_articles**](ArticlesApi.md#get_user_unpublished_articles) | **GET** /api/articles/me/unpublished | User's unpublished articles
[**unpublish_article**](ArticlesApi.md#unpublish_article) | **PUT** /api/articles/{id}/unpublish | Unpublish an article
[**update_article**](ArticlesApi.md#update_article) | **PUT** /api/articles/{id} | Update an article by id
[**videos**](ArticlesApi.md#videos) | **GET** /api/videos | Articles with a video



## create_article

> create_article(article)
Publish article

This endpoint allows the client to create a new article.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**article** | Option<[**Article**](Article.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_article_by_id

> Vec<crate::models::ArticleIndex> get_article_by_id(id)
Published article by id

This endpoint allows the client to retrieve a single published article given its `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::ArticleIndex>**](ArticleIndex.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_article_by_path

> Vec<crate::models::ArticleIndex> get_article_by_path(username, slug)
Published article by path

This endpoint allows the client to retrieve a single published article given its `path`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**slug** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ArticleIndex>**](ArticleIndex.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_articles

> Vec<crate::models::ArticleIndex> get_articles(page, per_page, tag, tags, tags_exclude, username, state, top, collection_id)
Published articles

This endpoint allows the client to retrieve a list of articles.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.  By default it will return featured, published articles ordered by descending popularity.  It supports pagination, each page will contain `30` articles by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]
**tag** | Option<**String**> | Using this parameter will retrieve articles that contain the requested tag. Articles will be ordered by descending popularity.This parameter can be used in conjuction with `top`. |  |
**tags** | Option<**String**> | Using this parameter will retrieve articles with any of the comma-separated tags. Articles will be ordered by descending popularity. |  |
**tags_exclude** | Option<**String**> | Using this parameter will retrieve articles that do _not_ contain _any_ of comma-separated tags. Articles will be ordered by descending popularity. |  |
**username** | Option<**String**> | Using this parameter will retrieve articles belonging             to a User or Organization ordered by descending publication date.             If `state=all` the number of items returned will be `1000` instead of the default `30`.             This parameter can be used in conjuction with `state`. |  |
**state** | Option<**String**> | Using this parameter will allow the client to check which articles are fresh or rising.             If `state=fresh` the server will return fresh articles.             If `state=rising` the server will return rising articles.             This param can be used in conjuction with `username`, only if set to `all`. |  |
**top** | Option<**i32**> | Using this parameter will allow the client to return the most popular articles in the last `N` days. `top` indicates the number of days since publication of the articles returned. This param can be used in conjuction with `tag`. |  |
**collection_id** | Option<**i32**> | Adding this will allow the client to return the list of articles belonging to the requested collection, ordered by ascending publication date. |  |

### Return type

[**Vec<crate::models::ArticleIndex>**](ArticleIndex.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_articles

> Vec<crate::models::ArticleIndex> get_latest_articles(page, per_page)
Published articles sorted by published date

This endpoint allows the client to retrieve a list of articles. ordered by descending publish date.  It supports pagination, each page will contain 30 articles by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_user_all_articles

> Vec<crate::models::ArticleIndex> get_user_all_articles(page, per_page)
User's all articles

This endpoint allows the client to retrieve a list of all articles on behalf of an authenticated user.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.  It will return both published and unpublished articles with pagination.  Unpublished articles will be at the top of the list in reverse chronological creation order. Published articles will follow in reverse chronological publication order.  By default a page will contain 30 articles.

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


## get_user_articles

> Vec<crate::models::ArticleIndex> get_user_articles(page, per_page)
User's articles

This endpoint allows the client to retrieve a list of published articles on behalf of an authenticated user.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.  Published articles will be in reverse chronological publication order.  It will return published articles with pagination. By default a page will contain 30 articles.

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


## get_user_published_articles

> Vec<crate::models::ArticleIndex> get_user_published_articles(page, per_page)
User's published articles

This endpoint allows the client to retrieve a list of published articles on behalf of an authenticated user.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.  Published articles will be in reverse chronological publication order.  It will return published articles with pagination. By default a page will contain 30 articles.

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


## get_user_unpublished_articles

> Vec<crate::models::ArticleIndex> get_user_unpublished_articles(page, per_page)
User's unpublished articles

This endpoint allows the client to retrieve a list of unpublished articles on behalf of an authenticated user.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.  Unpublished articles will be in reverse chronological creation order.  It will return unpublished articles with pagination. By default a page will contain 30 articles.

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


## unpublish_article

> unpublish_article(id, note)
Unpublish an article

This endpoint allows the client to unpublish an article.  The user associated with the API key must have any 'admin' or 'moderator' role.  The article will be unpublished and will no longer be visible to the public. It will remain in the database and will set back to draft status on the author's posts dashboard. Any notifications associated with the article will be deleted. Any comments on the article will remain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the article to unpublish. | [required] |
**note** | Option<**String**> | Content for the note that's created along with unpublishing |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_article

> update_article(id, article)
Update an article by id

This endpoint allows the client to update an existing article.  \"Articles\" are all the posts that users create on DEV that typically show up in the feed. They can be a blog post, a discussion question, a help thread etc. but is referred to as article within the code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user to unpublish. | [required] |
**article** | Option<[**Article**](Article.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos

> Vec<crate::models::VideoArticle> videos(page, per_page)
Articles with a video

This endpoint allows the client to retrieve a list of articles that are uploaded with a video.  It will only return published video articles ordered by descending popularity.  It supports pagination, each page will contain 24 articles by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 24]

### Return type

[**Vec<crate::models::VideoArticle>**](VideoArticle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

