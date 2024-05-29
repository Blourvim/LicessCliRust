# \BoardApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_board_seek**](BoardApi.md#api_board_seek) | **POST** /api/board/seek | Create a seek
[**api_stream_event**](BoardApi.md#api_stream_event) | **GET** /api/stream/event | Stream incoming events
[**board_game_abort**](BoardApi.md#board_game_abort) | **POST** /api/board/game/{gameId}/abort | Abort a game
[**board_game_berserk**](BoardApi.md#board_game_berserk) | **POST** /api/board/game/{gameId}/berserk | Berserk a tournament game
[**board_game_chat_get**](BoardApi.md#board_game_chat_get) | **GET** /api/board/game/{gameId}/chat | Fetch the game chat
[**board_game_chat_post**](BoardApi.md#board_game_chat_post) | **POST** /api/board/game/{gameId}/chat | Write in the chat
[**board_game_claim_victory**](BoardApi.md#board_game_claim_victory) | **POST** /api/board/game/{gameId}/claim-victory | Claim victory of a game
[**board_game_draw**](BoardApi.md#board_game_draw) | **POST** /api/board/game/{gameId}/draw/{accept} | Handle draw offers
[**board_game_move**](BoardApi.md#board_game_move) | **POST** /api/board/game/{gameId}/move/{move} | Make a Board move
[**board_game_resign**](BoardApi.md#board_game_resign) | **POST** /api/board/game/{gameId}/resign | Resign a game
[**board_game_stream**](BoardApi.md#board_game_stream) | **GET** /api/board/game/stream/{gameId} | Stream Board game state
[**board_game_takeback**](BoardApi.md#board_game_takeback) | **POST** /api/board/game/{gameId}/takeback/{accept} | Handle takeback offers



## api_board_seek

> api_board_seek(rated, time, increment, days, variant, color, rating_range)
Create a seek

   Create a public seek, to start a game with a random player.    ### Real-time seek    Specify the `time` and `increment` clock values.  The response is streamed but doesn't contain any information.    **Keep the connection open to keep the seek active**.    If the client closes the connection, the seek is canceled. This way, if the client terminates, the user won't be paired in a game they wouldn't play.   When the seek is accepted, or expires, the server closes the connection.    **Make sure to also have an [Event stream](#operation/apiStreamEvent) open**, to be notified when a game starts.   We recommend opening the [Event stream](#operation/apiStreamEvent) first, then the seek stream. This way,   you won't miss the game event if the seek is accepted immediately.    ### Correspondence seek    Specify the `days` per turn value.  The response is not streamed, it immediately completes with the seek ID. The seek remains active on the server until it is joined by someone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rated** | Option<**bool**> | Whether the game is rated and impacts players ratings. |  |[default to false]
**time** | Option<**f32**> | Clock initial time in minutes. Required for real-time seeks. |  |
**increment** | Option<**i32**> | Clock increment in seconds. Required for real-time seeks. |  |
**days** | Option<**i32**> | Days per turn. Required for correspondence seeks. |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**color** | Option<**String**> | The color to play. Better left empty to automatically get 50% white. |  |[default to random]
**rating_range** | Option<**String**> | The rating range of potential opponents. Better left empty. Example: 1500-1800  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_stream_event

> crate::models::ApiStreamEvent200Response api_stream_event()
Stream incoming events

   Stream the events reaching a lichess user in real time as [ndjson](#section/Introduction/Streaming-with-ND-JSON).    An empty line is sent every 6 seconds for keep alive purposes.    Each non-empty line is a JSON object containing a `type` field. Possible values are:   - `gameStart` Start of a game   - `gameFinish` Completion of a game   - `challenge` A player sends you a challenge or you challenge someone   - `challengeCanceled` A player cancels their challenge to you   - `challengeDeclined` The opponent declines your challenge     When the stream opens, all current challenges and games are sent.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiStreamEvent200Response**](apiStreamEvent_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_abort

> crate::models::Ok board_game_abort(game_id)
Abort a game

Abort a game being played with the Board API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_berserk

> crate::models::Ok board_game_berserk(game_id)
Berserk a tournament game

Go berserk on an arena tournament game. Halves the clock time, grants an extra point upon winning. Only available in arena tournaments that allow berserk, and before each player has made a move. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_chat_get

> serde_json::Value board_game_chat_get(game_id)
Fetch the game chat

Get the messages posted in the game chat 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_chat_post

> crate::models::Ok board_game_chat_post(game_id, room, text)
Write in the chat

Post a message to the player or spectator chat, in a game being played with the Board API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**room** | **String** |  | [required] |
**text** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_claim_victory

> crate::models::Ok board_game_claim_victory(game_id)
Claim victory of a game

Claim victory when the opponent has left the game for a while. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_draw

> crate::models::Ok board_game_draw(game_id, accept)
Handle draw offers

Create/accept/decline draw offers. - `yes`: Offer a draw, or accept the opponent's draw offer. - `no`: Decline a draw offer from the opponent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**accept** | [**BoardGameDrawAcceptParameter**](.md) |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_move

> crate::models::Ok board_game_move(game_id, r#move, offering_draw)
Make a Board move

Make a move in a game being played with the Board API.  The move can also contain a draw offer/agreement. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**r#move** | **String** | The move to play, in UCI format | [required] |
**offering_draw** | Option<**bool**> | Whether to offer (or agree to) a draw |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_resign

> crate::models::Ok board_game_resign(game_id)
Resign a game

Resign a game being played with the Board API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_stream

> crate::models::BoardGameStream200Response board_game_stream(game_id)
Stream Board game state

 Stream the state of a game being played with the Board API, as [ndjson](#section/Introduction/Streaming-with-ND-JSON).  Use this endpoint to get updates about the game in real-time, with a single request.  Each line is a JSON object containing a `type` field. Possible values are:   - `gameFull` Full game data. All values are immutable, except for the `state` field.   - `gameState` Current state of the game. Immutable values not included. Sent when a move is played, a draw is offered, or when the game ends.   - `chatLine` Chat message sent by a user in the `room` \"player\" or \"spectator\".    - `opponentGone` Whether the opponent has left the game, and how long before you can claim a win or draw.    The first line is always of type `gameFull`.    The server closes the stream when the game ends, or if the game has already ended.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::BoardGameStream200Response**](boardGameStream_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_game_takeback

> crate::models::Ok board_game_takeback(game_id, accept)
Handle takeback offers

Create/accept/decline takebacks. - `yes`: Propose a takeback, or accept the opponent's takeback offer. - `no`: Decline a takeback offer from the opponent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**accept** | [**BoardGameDrawAcceptParameter**](.md) |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

