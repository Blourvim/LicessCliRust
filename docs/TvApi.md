# \TvApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tv_channel_games**](TvApi.md#tv_channel_games) | **GET** /api/tv/{channel} | Get best ongoing games of a TV channel
[**tv_channels**](TvApi.md#tv_channels) | **GET** /api/tv/channels | Get current TV games
[**tv_feed**](TvApi.md#tv_feed) | **GET** /api/tv/feed | Stream current TV game



## tv_channel_games

> serde_json::Value tv_channel_games(channel, nb, moves, pgn_in_json, tags, clocks, opening)
Get best ongoing games of a TV channel

Get a list of ongoing games for a given TV channel. Similar to [lichess.org/games](https://lichess.org/games).  Available in PGN or [ndjson](#section/Introduction/Streaming-with-ND-JSON) format, depending on the request `Accept` header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** | The name of the channel in camel case. | [required] |
**nb** | Option<**f32**> | Number of games to fetch. |  |[default to 10]
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn, application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tv_channels

> serde_json::Value tv_channels()
Get current TV games

Get basic info about the best games being played for each speed and variant, but also computer games and bot games.  See [lichess.org/tv](https://lichess.org/tv). 

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


## tv_feed

> serde_json::Value tv_feed()
Stream current TV game

Stream positions and moves of the current [TV game](https://lichess.org/tv) in [ndjson](#section/Introduction/Streaming-with-ND-JSON). A summary of the game is sent as a first message, and when the featured game changes.  Try it with `curl https://lichess.org/api/tv/feed`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

