# \ChallengesApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_challenge_tokens**](ChallengesApi.md#admin_challenge_tokens) | **POST** /api/token/admin-challenge | Admin challenge tokens
[**challenge_accept**](ChallengesApi.md#challenge_accept) | **POST** /api/challenge/{challengeId}/accept | Accept a challenge
[**challenge_ai**](ChallengesApi.md#challenge_ai) | **POST** /api/challenge/ai | Challenge the AI
[**challenge_cancel**](ChallengesApi.md#challenge_cancel) | **POST** /api/challenge/{challengeId}/cancel | Cancel a challenge
[**challenge_create**](ChallengesApi.md#challenge_create) | **POST** /api/challenge/{username} | Create a challenge
[**challenge_decline**](ChallengesApi.md#challenge_decline) | **POST** /api/challenge/{challengeId}/decline | Decline a challenge
[**challenge_list**](ChallengesApi.md#challenge_list) | **GET** /api/challenge | List your challenges
[**challenge_open**](ChallengesApi.md#challenge_open) | **POST** /api/challenge/open | Open-ended challenge
[**challenge_start_clocks**](ChallengesApi.md#challenge_start_clocks) | **POST** /api/challenge/{gameId}/start-clocks | Start clocks of a game
[**round_add_time**](ChallengesApi.md#round_add_time) | **POST** /api/round/{gameId}/add-time/{seconds} | Add time to the opponent clock



## admin_challenge_tokens

> admin_challenge_tokens(users, description)
Admin challenge tokens

**This endpoint can only be used by Lichess administrators. It will not work if you do not have the appropriate permissions.** Tournament organizers should instead use [OAuth](#tag/OAuth) to obtain `challenge:write` tokens from users in order to perform bulk pairing.*  Create and obtain `challenge:write` tokens for multiple users.  If a similar token already exists for a user, it is reused. This endpoint is idempotent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users** | **String** | Usernames separated with commas | [required] |
**description** | **String** | User visible description of the token | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_accept

> crate::models::Ok challenge_accept(challenge_id)
Accept a challenge

Accept an incoming challenge.  You should receive a `gameStart` event on the [incoming events stream](#operation/apiStreamEvent). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_ai

> crate::models::GameJson challenge_ai(level, clock_period_limit, clock_period_increment, days, color, variant, fen)
Challenge the AI

Start a game with Lichess AI.  You will be notified on the [event stream](#operation/apiStreamEvent) that a new game has started. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**level** | Option<**f32**> | AI strength |  |
**clock_period_limit** | Option<**f32**> | Clock initial time in seconds. If empty, a correspondence game is created. |  |
**clock_period_increment** | Option<**i32**> | Clock increment in seconds. If empty, a correspondence game is created. |  |
**days** | Option<**i32**> | Days per move, for correspondence games. Clock settings must be omitted. |  |
**color** | Option<**String**> | Which color you get to play |  |[default to random]
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**fen** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |

### Return type

[**crate::models::GameJson**](GameJson.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_cancel

> crate::models::Ok challenge_cancel(challenge_id, opponent_token)
Cancel a challenge

Cancel a challenge you sent, or aborts the game if the challenge was accepted, but the game was not yet played. Note that the ID of a game is the same as the ID of the challenge that created it.  Works for user challenges and open challenges alike. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_id** | **String** |  | [required] |
**opponent_token** | Option<**String**> | Optional `challenge:write` token of the opponent. If set, the game can be canceled even if both players have moved. |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_create

> crate::models::ChallengeCreate200Response challenge_create(username, rated, clock_period_limit, clock_period_increment, days, color, variant, fen, keep_alive_stream, rules)
Create a challenge

Challenge someone to play. The targeted player can choose to accept or decline.  If the challenge is accepted, you will be notified on the [event stream](#operation/apiStreamEvent) that a new game has started. The game ID will be the same as the challenge ID.  Challenges for realtime games (not correspondence) expire after 20s if not accepted. To prevent that, use the `keepAliveStream` flag described below. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**rated** | Option<**bool**> | Game is rated and impacts players ratings |  |[default to false]
**clock_period_limit** | Option<**f32**> | Clock initial time in seconds. If empty, a correspondence game is created. Valid values are 0, 15, 30, 45, 60, 90, and any multiple of 60 up to 10800 (3 hours). |  |
**clock_period_increment** | Option<**i32**> | Clock increment in seconds. If empty, a correspondence game is created. |  |
**days** | Option<**i32**> | Days per move, for correspondence games. Clock settings must be omitted. |  |
**color** | Option<**String**> | Which color you get to play |  |[default to random]
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**fen** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**keep_alive_stream** | Option<**bool**> | If set, the response is streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). The challenge is kept alive until the connection is closed by the client. When the challenge is accepted, declined or canceled, a message of the form `{\\\"done\\\":\\\"accepted\\\"}` is sent, then the connection is closed by the server. If not set, the response is not streamed, and the challenge expires after 20s if not accepted.  |  |
**rules** | Option<**String**> | Extra game rules separated by commas. Example: `noAbort,noRematch`  |  |

### Return type

[**crate::models::ChallengeCreate200Response**](challengeCreate_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_decline

> crate::models::Ok challenge_decline(challenge_id, reason)
Decline a challenge

Decline an incoming challenge. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_id** | **String** |  | [required] |
**reason** | Option<**String**> | Reason challenge was declined. It will be translated to the player's language. See [the full list in the translation file](https://github.com/ornicar/lila/blob/master/translation/source/challenge.xml#L14). |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_list

> crate::models::ChallengeList200Response challenge_list()
List your challenges

Get a list of challenges created by or targeted at you. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ChallengeList200Response**](challengeList_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_open

> serde_json::Value challenge_open(rated, clock_period_limit, clock_period_increment, days, variant, fen, name, rules, users, expires_at)
Open-ended challenge

Create a challenge that any 2 players can join.  Share the URL of the challenge. the first 2 players to click it will be paired for a game.  The response body also contains `whiteUrl` and `blackUrl`. You can control which color each player gets by giving them these URLs, instead of the main challenge URL.  Open challenges expire after 24h.  If the challenge creation is [authenticated with OAuth2](#section/Introduction/Authentication), then you can use the [challenge cancel endpoint](#operation/challengeCancel) to cancel it.  To directly pair 2 known players, use [this endpoint](#operation/bulkPairingList) instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rated** | Option<**bool**> | Game is rated and impacts players ratings |  |[default to false]
**clock_period_limit** | Option<**f32**> | Clock initial time in seconds. If empty, a correspondence game is created. |  |
**clock_period_increment** | Option<**i32**> | Clock increment in seconds. If empty, a correspondence game is created. |  |
**days** | Option<**i32**> | Days per turn. For correspondence challenges. |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**fen** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**name** | Option<**String**> | Optional name for the challenge, that players will see on the challenge page. |  |
**rules** | Option<**String**> | Extra game rules separated by commas. Example: `noRematch,noGiveTime` The `noAbort` rule is available for Lichess admins only  |  |
**users** | Option<**String**> | Optional pair of usernames, separated by a comma. If set, only these users will be allowed to join the game. The first username gets the white pieces. Example: `Username1,Username2`  |  |
**expires_at** | Option<**i32**> | Timestamp in milliseconds to expire the challenge. Defaults to 24h after creation. Can't be more than 2 weeks after creation. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## challenge_start_clocks

> crate::models::Ok challenge_start_clocks(game_id, token1, token2)
Start clocks of a game

Start the clocks of a game immediately, even if a player has not yet made a move.  Requires the OAuth tokens of both players with `challenge:write` scope.  If the clocks have already started, the call will have no effect. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**token1** | Option<**String**> | OAuth token of a player |  |
**token2** | Option<**String**> | OAuth token of the other player |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## round_add_time

> crate::models::Ok round_add_time(game_id, seconds)
Add time to the opponent clock

Add seconds to the opponent's clock. Can be used to create games with time odds. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**seconds** | **String** | How many seconds to give | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

