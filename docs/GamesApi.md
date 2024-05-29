# \GamesApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_account_playing**](GamesApi.md#api_account_playing) | **GET** /api/account/playing | Get my ongoing games
[**api_games_user**](GamesApi.md#api_games_user) | **GET** /api/games/user/{username} | Export games of a user
[**api_imported_games_user**](GamesApi.md#api_imported_games_user) | **GET** /api/games/export/imports | Export your imported games
[**api_user_current_game**](GamesApi.md#api_user_current_game) | **GET** /api/user/{username}/current-game | Export ongoing game of a user
[**game_import**](GamesApi.md#game_import) | **POST** /api/import | Import one game
[**game_pgn**](GamesApi.md#game_pgn) | **GET** /game/export/{gameId} | Export one game
[**games_by_ids**](GamesApi.md#games_by_ids) | **POST** /api/stream/games/{streamId} | Stream games by IDs
[**games_by_ids_add**](GamesApi.md#games_by_ids_add) | **POST** /api/stream/games/{streamId}/add | Add game IDs to stream
[**games_by_users**](GamesApi.md#games_by_users) | **POST** /api/stream/games-by-users | Stream games of users
[**games_export_ids**](GamesApi.md#games_export_ids) | **POST** /api/games/export/_ids | Export games by IDs
[**stream_game**](GamesApi.md#stream_game) | **GET** /api/stream/game/{id} | Stream moves of a game



## api_account_playing

> serde_json::Value api_account_playing(nb)
Get my ongoing games

Get the ongoing games of the current user. Real-time and correspondence games are included. The most urgent games are listed first. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nb** | Option<**i32**> | Max number of games to fetch |  |[default to 9]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_games_user

> serde_json::Value api_games_user(username, since, until, max, vs, rated, perf_type, color, analysed, moves, pgn_in_json, tags, clocks, evals, accuracy, opening, ongoing, finished, literate, last_fen, players, sort)
Export games of a user

Download all games of any user in PGN or [ndjson](#section/Introduction/Streaming-with-ND-JSON) format.  Games are sorted by reverse chronological order (most recent first).  We recommend streaming the response, for it can be very long. <https://lichess.org/@/german11> for instance has more than 500,000 games.  The game stream is throttled, depending on who is making the request:   - Anonymous request: 20 games per second   - [OAuth2 authenticated](#section/Introduction/Authentication) request: 30 games per second   - Authenticated, downloading your own games: 60 games per second 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The user name. | [required] |
**since** | Option<**i32**> | Download games played since this timestamp. Defaults to account creation date. |  |
**until** | Option<**i32**> | Download games played until this timestamp. Defaults to now. |  |
**max** | Option<**i32**> | How many games to download. Leave empty to download all games. |  |
**vs** | Option<**String**> | [Filter] Only games played against this opponent |  |
**rated** | Option<**bool**> | [Filter] Only rated (`true`) or casual (`false`) games |  |
**perf_type** | Option<[**crate::models::PerfType**](.md)> | [Filter] Only games in these speeds or variants.   Multiple perf types can be specified, separated by a comma.   Example: blitz,rapid,classical |  |
**color** | Option<**String**> | [Filter] Only games played as this color. |  |
**analysed** | Option<**bool**> | [Filter] Only games with or without a computer analysis available |  |
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. The response type must be set to `application/x-ndjson` by the request `Accept` header. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to false]
**evals** | Option<**bool**> | Include analysis evaluations and comments, when available.  Either as PGN comments: `12. Bxf6 { [%eval 0.23] } a3 { [%eval -1.09] }`  Or in an `analysis` JSON field, depending on the response type.  |  |[default to false]
**accuracy** | Option<**bool**> | Include [accuracy percent](https://lichess.org/page/accuracy) of each player, when available.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to false]
**ongoing** | Option<**bool**> | Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control, as to prevent cheat bots from using this API. |  |[default to false]
**finished** | Option<**bool**> | Include finished games. Set to `false` to only get ongoing games. |  |[default to true]
**literate** | Option<**bool**> | Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.  Example: `5... g4? { (-0.98 → 0.60) Mistake. Best move was h6. } (5... h6 6. d4 Ne7 7. g3 d5 8. exd5 fxg3 9. hxg3 c6 10. dxc6)`  |  |[default to false]
**last_fen** | Option<**bool**> | Include the FEN notation of the last position of the game.  The response type must be set to `application/x-ndjson` by the request `Accept` header.  |  |[default to false]
**players** | Option<**String**> | URL of a text file containing real names and ratings, to replace Lichess usernames and ratings in the PGN. Example: <https://gist.githubusercontent.com/ornicar/6bfa91eb61a2dcae7bcd14cce1b2a4eb/raw/768b9f6cc8a8471d2555e47ba40fb0095e5fba37/gistfile1.txt>  |  |
**sort** | Option<**String**> | Sort order of the games. |  |[default to dateDesc]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn, application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_imported_games_user

> serde_json::Value api_imported_games_user()
Export your imported games

Download all games imported by you. Games are exported in PGN format.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_user_current_game

> serde_json::Value api_user_current_game(username, moves, pgn_in_json, tags, clocks, evals, accuracy, opening, literate, players)
Export ongoing game of a user

Download the ongoing game, or the last game played, of a user. Available in either PGN or JSON format.  Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control, as to prevent cheat bots from using this API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to true]
**evals** | Option<**bool**> | Include analysis evaluations and comments, when available.  Either as PGN comments: `12. Bxf6 { [%eval 0.23] } a3 { [%eval -1.09] }`  Or in an `analysis` JSON field, depending on the response type.  |  |[default to true]
**accuracy** | Option<**bool**> | Include [accuracy percent](https://lichess.org/page/accuracy) of each player, when available.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to true]
**literate** | Option<**bool**> | Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.  Example: `5... g4? { (-0.98 → 0.60) Mistake. Best move was h6. } (5... h6 6. d4 Ne7 7. g3 d5 8. exd5 fxg3 9. hxg3 c6 10. dxc6)`  |  |[default to false]
**players** | Option<**String**> | URL of a text file containing real names and ratings, to replace Lichess usernames and ratings in the PGN. Example: <https://gist.githubusercontent.com/ornicar/6bfa91eb61a2dcae7bcd14cce1b2a4eb/raw/768b9f6cc8a8471d2555e47ba40fb0095e5fba37/gistfile1.txt>  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_import

> serde_json::Value game_import(pgn)
Import one game

Import a game from PGN. See <https://lichess.org/paste>.  Rate limiting: 200 games per hour for OAuth requests, 100 games per hour for anonymous requests.  To broadcast ongoing games, consider [pushing to a broadcast instead](#operation/broadcastPush).  To analyse a position or a line, just construct an analysis board URL: [https://lichess.org/analysis/pgn/e4_e5_Nf3_Nc6_Bc4_Bc5_Bxf7+](https://lichess.org/analysis/pgn/e4_e5_Nf3_Nc6_Bc4_Bc5_Bxf7+) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pgn** | Option<**String**> | The PGN. It can contain only one game. Most standard tags are supported. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_pgn

> serde_json::Value game_pgn(game_id, moves, pgn_in_json, tags, clocks, evals, accuracy, opening, literate, players)
Export one game

Download one game in either PGN or JSON format.  Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control, as to prevent cheat bots from using this API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | The game ID (8 characters). | [required] |
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to true]
**evals** | Option<**bool**> | Include analysis evaluations and comments, when available.  Either as PGN comments: `12. Bxf6 { [%eval 0.23] } a3 { [%eval -1.09] }`  Or in an `analysis` JSON field, depending on the response type.  |  |[default to true]
**accuracy** | Option<**bool**> | Include [accuracy percent](https://lichess.org/page/accuracy) of each player, when available.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to true]
**literate** | Option<**bool**> | Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.  Example: `5... g4? { (-0.98 → 0.60) Mistake. Best move was h6. } (5... h6 6. d4 Ne7 7. g3 d5 8. exd5 fxg3 9. hxg3 c6 10. dxc6)`  |  |[default to false]
**players** | Option<**String**> | URL of a text file containing real names and ratings, to replace Lichess usernames and ratings in the PGN. Example: <https://gist.githubusercontent.com/ornicar/6bfa91eb61a2dcae7bcd14cce1b2a4eb/raw/768b9f6cc8a8471d2555e47ba40fb0095e5fba37/gistfile1.txt>  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_ids

> serde_json::Value games_by_ids(stream_id, body)
Stream games by IDs

Creates a stream of games from an arbitrary streamId, and a list of game IDs.  The stream first outputs the games that already exists, then emits an event each time a game is started or finished.  Games are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON).  Maximum number of games: 500 for anonymous requests, or 1000 for [OAuth2 authenticated](#section/Introduction/Authentication) requests.  While the stream is open, it is possible to [add new game IDs to watch](#operation/gamesByIdsAdd). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** |  | [required] |
**body** | **String** | Up to 500 or 1000 game IDs separated by commas. Example: `gameId01,gameId02,gameId03`  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_ids_add

> crate::models::Ok games_by_ids_add(stream_id, body)
Add game IDs to stream

Add new game IDs for [an existing stream](#operation/gamesByIds) to watch. The stream will immediately outputs the games that already exists, then emit an event each time a game is started or finished. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** |  | [required] |
**body** | **String** | Up to 500 or 1000 game IDs separated by commas. Example: `gameId04,gameId05,gameId06`  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_users

> serde_json::Value games_by_users(body, with_current_games)
Stream games of users

Stream the games played between a list of users, in real time. Only games where **both players** are part of the list are included.  The stream emits an event each time a game is started or finished.  To also get all current ongoing games at the beginning of the stream, use the `withCurrentGames` flag.  Games are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON).  Maximum number of users: 300.  The method is `POST` so a longer list of IDs can be sent in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Up to 300 user IDs separated by commas. Example: `aliquantus,chess-network,lovlas`  | [required] |
**with_current_games** | Option<**bool**> | Include the already started games at the beginning of the stream. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_export_ids

> serde_json::Value games_export_ids(body, moves, pgn_in_json, tags, clocks, evals, accuracy, opening, literate, players)
Export games by IDs

Download games by IDs in PGN or [ndjson](#section/Introduction/Streaming-with-ND-JSON) format, depending on the request `Accept` header.  Games are sorted by reverse chronological order (most recent first)  The method is `POST` so a longer list of IDs can be sent in the request body.  300 IDs can be submitted.  Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control, as to prevent cheat bots from using this API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Game IDs separated by commas. Up to 300. | [required] |
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to false]
**evals** | Option<**bool**> | Include analysis evaluations and comments, when available.  Either as PGN comments: `12. Bxf6 { [%eval 0.23] } a3 { [%eval -1.09] }`  Or in an `analysis` JSON field, depending on the response type.  |  |[default to false]
**accuracy** | Option<**bool**> | Include [accuracy percent](https://lichess.org/page/accuracy) of each player, when available.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to false]
**literate** | Option<**bool**> | Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.  Example: `5... g4? { (-0.98 → 0.60) Mistake. Best move was h6. } (5... h6 6. d4 Ne7 7. g3 d5 8. exd5 fxg3 9. hxg3 c6 10. dxc6)`  |  |[default to false]
**players** | Option<**String**> | URL of a text file containing real names and ratings, to replace Lichess usernames and ratings in the PGN. Example: <https://gist.githubusercontent.com/ornicar/6bfa91eb61a2dcae7bcd14cce1b2a4eb/raw/768b9f6cc8a8471d2555e47ba40fb0095e5fba37/gistfile1.txt>  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/x-chess-pgn, application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_game

> serde_json::Value stream_game(id)
Stream moves of a game

Stream positions and moves of any ongoing game, in [ndjson](#section/Introduction/Streaming-with-ND-JSON).  A description of the game is sent as a first message. Then a message is sent each time a move is played. Finally a description of the game is sent when it finishes, and the stream is closed.  Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control, as to prevent cheat bots from using this API.  No more than 8 game streams can be opened at the same time from the same IP address. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

