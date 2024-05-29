# \BotApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_bot_online**](BotApi.md#api_bot_online) | **GET** /api/bot/online | Get online bots
[**api_stream_event**](BotApi.md#api_stream_event) | **GET** /api/stream/event | Stream incoming events
[**bot_account_upgrade**](BotApi.md#bot_account_upgrade) | **POST** /api/bot/account/upgrade | Upgrade to Bot account
[**bot_game_abort**](BotApi.md#bot_game_abort) | **POST** /api/bot/game/{gameId}/abort | Abort a game
[**bot_game_chat**](BotApi.md#bot_game_chat) | **POST** /api/bot/game/{gameId}/chat | Write in the chat
[**bot_game_chat_get**](BotApi.md#bot_game_chat_get) | **GET** /api/bot/game/{gameId}/chat | Fetch the game chat
[**bot_game_draw**](BotApi.md#bot_game_draw) | **POST** /api/bot/game/{gameId}/draw/{accept} | Handle draw offers in bot games
[**bot_game_move**](BotApi.md#bot_game_move) | **POST** /api/bot/game/{gameId}/move/{move} | Make a Bot move
[**bot_game_resign**](BotApi.md#bot_game_resign) | **POST** /api/bot/game/{gameId}/resign | Resign a game
[**bot_game_stream**](BotApi.md#bot_game_stream) | **GET** /api/bot/game/stream/{gameId} | Stream Bot game state



## api_bot_online

> crate::models::User api_bot_online(nb)
Get online bots

Stream the [online bot users](https://lichess.org/player/bots), as [ndjson](#section/Introduction/Streaming-with-ND-JSON). Throttled to 50 bot users per second.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nb** | Option<**i32**> | How many bot users to fetch |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

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


## bot_account_upgrade

> crate::models::Ok bot_account_upgrade()
Upgrade to Bot account

Upgrade a lichess player account into a Bot account. Only Bot accounts can use the Bot API.  The account **cannot have played any game** before becoming a Bot account. The upgrade is **irreversible**. The account will only be able to play as a Bot.  To upgrade an account to Bot, use the [official lichess-bot client](https://github.com/lichess-bot-devs/lichess-bot), or follow these steps: - Create an [API access token](https://lichess.org/account/oauth/token/create?scopes[]=bot:play) with \"Play bot moves\" permission. - `curl -d '' https://lichess.org/api/bot/account/upgrade -H \"Authorization: Bearer <yourTokenHere>\"`  To know if an account has already been upgraded, use the [Get my profile API](#operation/accountMe): the `title` field should be set to `BOT`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bot_game_abort

> crate::models::Ok bot_game_abort(game_id)
Abort a game

Abort a game being played with the Bot API. 

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


## bot_game_chat

> crate::models::Ok bot_game_chat(game_id, room, text)
Write in the chat

Post a message to the player or spectator chat, in a game being played with the Bot API. 

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


## bot_game_chat_get

> serde_json::Value bot_game_chat_get(game_id)
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


## bot_game_draw

> crate::models::Ok bot_game_draw(game_id, accept)
Handle draw offers in bot games

Create/accept/decline draw offers in bot games - `yes`: Offer a draw, or accept the opponent's draw offer. - `no`: Decline a draw offer from the opponent. 

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


## bot_game_move

> crate::models::Ok bot_game_move(game_id, r#move, offering_draw)
Make a Bot move

Make a move in a game being played with the Bot API.  The move can also contain a draw offer/agreement. 

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


## bot_game_resign

> crate::models::Ok bot_game_resign(game_id)
Resign a game

Resign a game being played with the Bot API. 

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


## bot_game_stream

> crate::models::BoardGameStream200Response bot_game_stream(game_id)
Stream Bot game state

 Stream the state of a game being played with the Bot API, as [ndjson](#section/Introduction/Streaming-with-ND-JSON).  Use this endpoint to get updates about the game in real-time, with a single request.  Each line is a JSON object containing a `type` field. Possible values are:   - `gameFull` Full game data. All values are immutable, except for the `state` field.   - `gameState` Current state of the game. Immutable values not included.   - `chatLine` Chat message sent by a user (or the bot itself) in the `room` \"player\" or \"spectator\".    - `opponentGone` Whether the opponent has left the game, and how long before you can claim a win or draw.    The first line is always of type `gameFull`.

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

