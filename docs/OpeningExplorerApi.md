# \OpeningExplorerApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**opening_explorer_lichess**](OpeningExplorerApi.md#opening_explorer_lichess) | **GET** /lichess | Lichess games
[**opening_explorer_master**](OpeningExplorerApi.md#opening_explorer_master) | **GET** /masters | Masters database
[**opening_explorer_master_game**](OpeningExplorerApi.md#opening_explorer_master_game) | **GET** /master/pgn/{gameId} | OTB master game
[**opening_explorer_player**](OpeningExplorerApi.md#opening_explorer_player) | **GET** /player | Player games



## opening_explorer_lichess

> serde_json::Value opening_explorer_lichess(variant, fen, play, speeds, ratings, since, until, moves, top_games, recent_games, history)
Lichess games

**Endpoint: <https://explorer.lichess.ovh/lichess>**  Games sampled from all Lichess players.  Example: `curl https://explorer.lichess.ovh/lichess?variant=standard&speeds=blitz,rapid,classical&ratings=2200,2500&fen=rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant** | Option<[**VariantKey**](.md)> | Variant |  |[default to standard]
**fen** | Option<**String**> | FEN or EPD of the root position |  |
**play** | Option<**String**> | Comma separated sequence of legal moves in UCI notation. Play additional moves starting from `fen`. Required to find an opening name, if `fen` is not an exact match for a named position.  |  |[default to ]
**speeds** | Option<[**Vec<crate::models::Speed>**](crate::models::Speed.md)> | Comma separated list of game speeds to filter by |  |
**ratings** | Option<[**Vec<f32>**](f32.md)> | Comma separated list of ratings groups to filter by. Each group ranges from its value to the next higher group in the enum (`0` from 0 to 999, `1000` from 1000 to 1199, ..., `2500` from 2500 to any rating above).  |  |
**since** | Option<**String**> | Include only games from this month or later |  |[default to 1952-01]
**until** | Option<**String**> | Include only games from this month or earlier |  |[default to 3000-12]
**moves** | Option<**f32**> | Number of most common moves to display |  |[default to 12]
**top_games** | Option<**f32**> | Number of top games to display |  |[default to 4]
**recent_games** | Option<**f32**> | Number of recent games to display |  |[default to 4]
**history** | Option<**bool**> | Optionally retrieve history |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opening_explorer_master

> serde_json::Value opening_explorer_master(fen, play, since, until, moves, top_games)
Masters database

**Endpoint: <https://explorer.lichess.ovh/masters>**  Example: `curl https://explorer.lichess.ovh/masters?play=d2d4,d7d5,c2c4,c7c6,c4d5` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fen** | Option<**String**> | FEN of the root position |  |
**play** | Option<**String**> | Comma separated sequence of legal moves in UCI notation. Play additional moves starting from `fen`. Required to find an opening name, if `fen` is not an exact match for a named position.  |  |[default to ]
**since** | Option<**f32**> | Include only games from this year or later |  |[default to 1952]
**until** | Option<**f32**> | Include only games from this year or earlier |  |
**moves** | Option<**f32**> | Number of most common moves to display |  |[default to 12]
**top_games** | Option<**f32**> | Number of top games to display |  |[default to 15]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opening_explorer_master_game

> serde_json::Value opening_explorer_master_game(game_id)
OTB master game

**Endpoint: `https://explorer.lichess.ovh/masters/pgn/{gameId}`**  Example: `curl https://explorer.lichess.ovh/masters/pgn/aAbqI4ey` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opening_explorer_player

> serde_json::Value opening_explorer_player(player, variant, fen, play, speeds, modes, since, until, moves, recent_games)
Player games

**Endpoint: <https://explorer.lichess.ovh/player>**  Games of a Lichess player.  Responds with a stream of [newline delimited JSON](#section/Introduction/Streaming-with-ND-JSON). Will start indexing on demand, immediately respond with the current results, and stream more updates until indexing is complete. The stream is throttled and deduplicated. Empty lines may be sent to avoid timeouts.  Will index new games at most once per minute, and revisit previously ongoing games at most once every day.  Example: `curl https://explorer.lichess.ovh/player?player=revoof&color=white&play=d2d4,d7d5&recentGames=1` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player** | Option<**String**> | Username or ID of the player |  |
**variant** | Option<[**VariantKey**](.md)> | Variant |  |[default to standard]
**fen** | Option<**String**> | FEN of the root position |  |
**play** | Option<**String**> | Comma separated sequence of legal moves in UCI notation. Play additional moves starting from `fen`. Required to find an opening name, if `fen` is not an exact match for a named position.  |  |[default to ]
**speeds** | Option<[**Vec<crate::models::Speed>**](crate::models::Speed.md)> | Comma separated list of game speeds to look for |  |
**modes** | Option<[**Vec<String>**](String.md)> | Comma separated list of modes |  |
**since** | Option<**String**> | Include only games from this month or later |  |[default to 1952-01]
**until** | Option<**String**> | Include only games from this month or earlier |  |[default to 3000-12]
**moves** | Option<**f32**> | Number of most common moves to display |  |
**recent_games** | Option<**f32**> | Number of recent games to display |  |[default to 8]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/nd-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

