# \PuzzlesApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_puzzle_activity**](PuzzlesApi.md#api_puzzle_activity) | **GET** /api/puzzle/activity | Get your puzzle activity
[**api_puzzle_daily**](PuzzlesApi.md#api_puzzle_daily) | **GET** /api/puzzle/daily | Get the daily puzzle
[**api_puzzle_dashboard**](PuzzlesApi.md#api_puzzle_dashboard) | **GET** /api/puzzle/dashboard/{days} | Get your puzzle dashboard
[**api_puzzle_id**](PuzzlesApi.md#api_puzzle_id) | **GET** /api/puzzle/{id} | Get a puzzle by its ID
[**api_storm_dashboard**](PuzzlesApi.md#api_storm_dashboard) | **GET** /api/storm/dashboard/{username} | Get the storm dashboard of a player
[**racer_post**](PuzzlesApi.md#racer_post) | **POST** /api/racer | Create and join a puzzle race



## api_puzzle_activity

> crate::models::PuzzleRoundJson api_puzzle_activity(max, before)
Get your puzzle activity

Download your puzzle activity in [ndjson](#section/Introduction/Streaming-with-ND-JSON) format.  Puzzle activity is sorted by reverse chronological order (most recent first)  We recommend streaming the response, for it can be very long. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | How many entries to download. Leave empty to download all activity. |  |
**before** | Option<**i32**> | Download entries before this timestamp. Defaults to now. Use `before` and `max` for pagination. |  |

### Return type

[**crate::models::PuzzleRoundJson**](PuzzleRoundJson.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_puzzle_daily

> serde_json::Value api_puzzle_daily()
Get the daily puzzle

Get the daily Lichess puzzle in JSON format.  Alternatively, you can [post it in your slack workspace](https://lichess.org/daily-puzzle-slack). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_puzzle_dashboard

> serde_json::Value api_puzzle_dashboard(days)
Get your puzzle dashboard

Download your [puzzle dashboard](https://lichess.org/training/dashboard/30/dashboard) as JSON.  Also includes all puzzle themes played, with aggregated results.  Allows re-creating the [improvement/strengths](https://lichess.org/training/dashboard/30/improvementAreas) interfaces. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | **i32** | How many days to look back when aggregating puzzle results. 30 is sensible. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_puzzle_id

> serde_json::Value api_puzzle_id(id)
Get a puzzle by its ID

Get a single Lichess puzzle in JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The puzzle ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_storm_dashboard

> serde_json::Value api_storm_dashboard(username, days)
Get the storm dashboard of a player

Download the [storm dashboard](https://lichess.org/storm/dashboard/mrbasso) of any player as JSON.  Contains the aggregated highscores, and the history of storm runs aggregated by days.  Use `?days=0` if you only care about the highscores. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username of the player | [required] |
**days** | Option<**i32**> | How many days of history to return |  |[default to 30]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racer_post

> serde_json::Value racer_post()
Create and join a puzzle race

Create a new private [puzzle race](https://lichess.org/racer). The Lichess user who creates the race must join the race page, and manually start the race when enough players have joined.  - <https://lichess.org/racer> 

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

