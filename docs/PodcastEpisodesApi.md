# \PodcastEpisodesApi

All URIs are relative to *https://dev.to/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_podcast_episodes**](PodcastEpisodesApi.md#get_podcast_episodes) | **GET** /api/podcast_episodes | Podcast Episodes



## get_podcast_episodes

> Vec<crate::models::PodcastEpisodeIndex> get_podcast_episodes(page, per_page, username)
Podcast Episodes

This endpoint allows the client to retrieve a list of podcast episodes.         \"Podcast episodes\" are episodes belonging to podcasts.         It will only return active (reachable) podcast episodes that belong to published podcasts available on the platform, ordered by descending publication date.         It supports pagination, each page will contain 30 articles by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Pagination page |  |[default to 1]
**per_page** | Option<**i32**> | Page size (the number of items to return per page). The default maximum value can be overridden by \"API_PER_PAGE_MAX\" environment variable. |  |[default to 30]
**username** | Option<**String**> | Using this parameter will retrieve episodes belonging to a specific podcast. |  |

### Return type

[**Vec<crate::models::PodcastEpisodeIndex>**](PodcastEpisodeIndex.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

