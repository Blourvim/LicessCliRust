# \BulkPairingsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_pairing_create**](BulkPairingsApi.md#bulk_pairing_create) | **POST** /api/bulk-pairing | Create a bulk pairing
[**bulk_pairing_delete**](BulkPairingsApi.md#bulk_pairing_delete) | **DELETE** /api/bulk-pairing/{id} | Cancel a bulk pairing
[**bulk_pairing_get**](BulkPairingsApi.md#bulk_pairing_get) | **GET** /api/bulk-pairing/{id} | Show a bulk pairing
[**bulk_pairing_list**](BulkPairingsApi.md#bulk_pairing_list) | **GET** /api/bulk-pairing | View your bulk pairings
[**bulk_pairing_start_clocks**](BulkPairingsApi.md#bulk_pairing_start_clocks) | **POST** /api/bulk-pairing/{id}/start-clocks | Manually start clocks



## bulk_pairing_create

> serde_json::Value bulk_pairing_create(players, clock_period_limit, clock_period_increment, days, pair_at, start_clocks_at, rated, variant, fen, message, rules)
Create a bulk pairing

Schedule many games at once, up to 24h in advance.  OAuth tokens are required for all paired players, with the `challenge:write` scope.  You can schedule up to 500 games every 10 minutes. [Contact us](mailto:contact@lichess.org) if you need higher limits.  If games have a real-time clock, each player must have only one pairing. For correspondence games, players can have multiple pairings within the same bulk.  The entire bulk is rejected if:   - a token is missing   - a token is present more than once (except in correspondence)   - a token lacks the `challenge:write` scope   - a player account is closed   - a player is paired more than once (except in correspondence)   - a bulk is already scheduled to start at the same time with the same player   - you have 20 scheduled bulks   - you have 1000 scheduled games  Partial bulks are never created. Either it all fails, or it all succeeds. When it fails, it does so with an error message explaining the issue. Failed bulks are not counted in the rate limiting, they are free. Fix the issues, manually or programmatically, then retry to schedule the bulk.  A successful bulk creation returns a JSON bulk document. Its ID can be used for further operations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**players** | Option<**String**> | OAuth tokens of all the players to pair, with the syntax `tokenOfWhitePlayerInGame1:tokenOfBlackPlayerInGame1,tokenOfWhitePlayerInGame2:tokenOfBlackPlayerInGame2,...`.  The 2 tokens of the players of a game are separated with `:`. The first token gets the white pieces. Games are separated with `,`.  Up to 1000 tokens can be sent, for a max of 500 games.  Each token must be included at most once.  Example: `token1:token2,token3:token4,token5:token6`  |  |
**clock_period_limit** | Option<**f32**> | Clock initial time in seconds. Example: `600`  |  |
**clock_period_increment** | Option<**i32**> | Clock increment in seconds. Example: `2`  |  |
**days** | Option<**i32**> | Days per turn. For correspondence games only. |  |
**pair_at** | Option<**i32**> | Date at which the games will be created as a Unix timestamp in milliseconds. Up to 7 days in the future. Omit, or set to current date and time, to start the games immediately. Example: `1612289869919`  |  |
**start_clocks_at** | Option<**i32**> | Date at which the clocks will be automatically started as a Unix timestamp in milliseconds. Up to 7 days in the future. Note that the clocks can start earlier than specified, if players start making moves in the game. If omitted, the clocks will not start automatically. Example: `1612289869919`  |  |
**rated** | Option<**bool**> | Game is rated and impacts players ratings |  |[default to false]
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**fen** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**message** | Option<**String**> | Message that will be sent to each player, when the game is created.  It is sent from your user account.  `{opponent}` and `{game}` are placeholders that will be replaced with the opponent and the game URLs.  You can omit this field to send the default message, but if you set your own message, it must at least contain the `{game}` placeholder.  |  |[default to Your game with {opponent} is ready: {game}.]
**rules** | Option<**String**> | Extra game rules separated by commas. Example: `noAbort,noRematch`  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_pairing_delete

> crate::models::Ok bulk_pairing_delete(id)
Cancel a bulk pairing

Cancel and delete a bulk pairing that is scheduled in the future.  If the games have already been created, then this does nothing.  Canceling a bulk pairing does not refund the rate limit cost of that bulk pairing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_pairing_get

> serde_json::Value bulk_pairing_get(id)
Show a bulk pairing

Get a single bulk pairing by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_pairing_list

> Vec<serde_json::Value> bulk_pairing_list()
View your bulk pairings

Get a list of bulk pairings you created. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_pairing_start_clocks

> crate::models::Ok bulk_pairing_start_clocks(id)
Manually start clocks

Immediately start all clocks of the games of a bulk pairing.  This overrides the `startClocksAt` value of an existing bulk pairing.  If the games have not yet been created (`bulk.pairAt` is in the future), then this does nothing.  If the clocks have already started (`bulk.startClocksAt` is in the past), then this does nothing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

