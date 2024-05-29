# \BroadcastsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broadcast_all_rounds_pgn**](BroadcastsApi.md#broadcast_all_rounds_pgn) | **GET** /api/broadcast/{broadcastTournamentId}.pgn | Export all rounds as PGN
[**broadcast_index**](BroadcastsApi.md#broadcast_index) | **GET** /api/broadcast | Get official broadcasts
[**broadcast_my_rounds_get**](BroadcastsApi.md#broadcast_my_rounds_get) | **GET** /api/broadcast/my-rounds | Get your broadcast rounds
[**broadcast_push**](BroadcastsApi.md#broadcast_push) | **POST** /broadcast/round/{broadcastRoundId}/push | Push PGN to your broadcast round
[**broadcast_round_create**](BroadcastsApi.md#broadcast_round_create) | **POST** /broadcast/{broadcastTournamentId}/new | Create a broadcast round
[**broadcast_round_get**](BroadcastsApi.md#broadcast_round_get) | **GET** /api/broadcast/{broadcastTournamentSlug}/{broadcastRoundSlug}/{broadcastRoundId} | Get a broadcast round
[**broadcast_round_pgn**](BroadcastsApi.md#broadcast_round_pgn) | **GET** /api/broadcast/round/{broadcastRoundId}.pgn | Export one round as PGN
[**broadcast_round_update**](BroadcastsApi.md#broadcast_round_update) | **POST** /broadcast/round/{broadcastRoundId}/edit | Update your broadcast round
[**broadcast_stream_round_pgn**](BroadcastsApi.md#broadcast_stream_round_pgn) | **GET** /api/stream/broadcast/round/{broadcastRoundId}.pgn | Stream an ongoing broadcast tournament as PGN
[**broadcast_tour_create**](BroadcastsApi.md#broadcast_tour_create) | **POST** /broadcast/new | Create a broadcast tournament
[**broadcast_tour_get**](BroadcastsApi.md#broadcast_tour_get) | **GET** /broadcast/{slug}/{broadcastTournamentId} | Get your broadcast tournament
[**broadcast_tour_update**](BroadcastsApi.md#broadcast_tour_update) | **POST** /broadcast/{broadcastTournamentId}/edit | Update your broadcast tournament



## broadcast_all_rounds_pgn

> serde_json::Value broadcast_all_rounds_pgn(broadcast_tournament_id)
Export all rounds as PGN

Download all games of all rounds of a broadcast in PGN format.  If a `study:read` [OAuth token](#tag/OAuth) is provided, the private rounds where the user is a contributor will be available.  You may want to [download only the games of a single round](#operation/broadcastRoundPgn) instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_tournament_id** | **String** | The broadcast tournament ID (8 characters). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_index

> Vec<serde_json::Value> broadcast_index(nb)
Get official broadcasts

Get all incoming, ongoing, and finished official broadcasts. The broadcasts are sorted by start date, most recent first.  Broadcasts are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nb** | Option<**i32**> | Max number of broadcasts to fetch |  |[default to 20]

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_my_rounds_get

> serde_json::Value broadcast_my_rounds_get(nb)
Get your broadcast rounds

Stream all broadcast rounds you are a member of. Also includes broadcasts rounds you did not create, but were invited to. Also includes broadcasts rounds where you're a non-writing member. See the `writeable` flag in the response. Rounds are ordered by rank, which is roughly chronological, most recent first, slightly pondered with popularity. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nb** | Option<**i32**> | How many rounds to get |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_push

> crate::models::BroadcastPush200Response broadcast_push(broadcast_round_id, body)
Push PGN to your broadcast round

Update your broadcast with new PGN. Only for broadcast without a source URL. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_round_id** | **String** | The broadcast round ID (8 characters). | [required] |
**body** | **String** | The PGN. It can contain up to 64 games, separated by a double new line. | [required] |

### Return type

[**crate::models::BroadcastPush200Response**](broadcastPush_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_round_create

> serde_json::Value broadcast_round_create(broadcast_tournament_id, name, sync_url, sync_url_round, starts_at, delay, period, finished)
Create a broadcast round

Create a new broadcast round to relay external games. This endpoint accepts the same form data as the web form. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_tournament_id** | **String** | The broadcast tournament ID (8 characters). | [required] |
**name** | **String** | Name of the broadcast round. Length must be between 3 and 80 characters.  Example: `Round 1`  | [required] |
**sync_url** | Option<**String**> | URL that Lichess will poll to get updates about the games. It must be publicly accessible from the Internet.  Example: `https://myserver.org/myevent/round-10/games.pgn`  If the syncUrl is missing, then the broadcast needs to be fed by [pushing PGN to it](#operation/broadcastPush).  |  |
**sync_url_round** | Option<**String**> | Required if `syncUrl` contains a livechesscloud link.  |  |
**starts_at** | Option<**i32**> | Timestamp in milliseconds of broadcast round start. Leave empty to manually start the broadcast round.  Example: `1356998400070`  |  |
**delay** | Option<**i32**> | Delay in seconds for movements to appear on the broadcast. Leave it empty if you don't need it.  Example: `900` (15 min)  |  |
**period** | Option<**i32**> | (Only for Admins) Waiting time for each poll.  |  |
**finished** | Option<**bool**> | Mark whether the round has been completed.  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_round_get

> serde_json::Value broadcast_round_get(broadcast_tournament_slug, broadcast_round_slug, broadcast_round_id)
Get a broadcast round

Get information about a broadcast round. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_tournament_slug** | **String** | The broadcast tournament slug. Only used for SEO, the slug can be safely replaced by `-`. Only the `broadcastRoundId` is actually used. | [required] |
**broadcast_round_slug** | **String** | The broadcast round slug. Only used for SEO, the slug can be safely replaced by `-`. Only the `broadcastRoundId` is actually used. | [required] |
**broadcast_round_id** | **String** | The broadcast Round ID (8 characters). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_round_pgn

> serde_json::Value broadcast_round_pgn(broadcast_round_id)
Export one round as PGN

Download all games of a single round of a broadcast tournament in PGN format.  You *could* poll this endpoint to get updates about a tournament, but it would be slow, and very inneficient.  Instead, consider [streaming the tournament](#operation/broadcastStreamRoundPgn) to get a new PGN every time a game is updated, in real-time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_round_id** | **String** | The round ID (8 characters). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_round_update

> crate::models::Ok broadcast_round_update(broadcast_round_id, name, sync_url, sync_url_round, starts_at, delay, period, finished)
Update your broadcast round

Update information about a broadcast round that you created. This endpoint accepts the same form data as the web form. All fields must be populated with data. Missing fields will override the broadcast with empty data. For instance, if you omit `startDate`, then any pre-existing start date will be removed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_round_id** | **String** | The broadcast round ID (8 characters). | [required] |
**name** | **String** | Name of the broadcast round. Length must be between 3 and 80 characters.  Example: `Round 10`  | [required] |
**sync_url** | Option<**String**> | URL that Lichess will poll to get updates about the games. It must be publicly accessible from the Internet.  Example: `https://myserver.org/myevent/round-10/games.pgn`  |  |
**sync_url_round** | Option<**String**> | Required if `syncUrl` contains a livechesscloud link.  |  |
**starts_at** | Option<**i32**> | Timestamp in milliseconds of broadcast start. Leave empty to manually start the broadcast.  Example: `1356998400070`  |  |
**delay** | Option<**i32**> | Delay in seconds for movements to appear on the broadcast. Leave it empty if you don't need it.  Example: `900` (15 min)  |  |
**period** | Option<**i32**> | (Only for Admins) Waiting time for each poll.  |  |
**finished** | Option<**bool**> | Mark whether the round has been completed.  |  |[default to false]

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_stream_round_pgn

> serde_json::Value broadcast_stream_round_pgn(broadcast_round_id)
Stream an ongoing broadcast tournament as PGN

This streaming endpoint first sends all games of a broadcast tournament in PGN format.  Then, it waits for new moves to be played. As soon as it happens, the entire PGN of the game is sent to the stream.  The stream will also send PGNs when games are added to the tournament.  This is the best way to get updates about an ongoing tournament. Streaming means no polling, and no pollings means no latency, and minimum impact on the server. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_round_id** | **String** | The broadcast round ID (8 characters). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_tour_create

> serde_json::Value broadcast_tour_create(name, description, auto_leaderboard, markdown, tier, players)
Create a broadcast tournament

Create a new broadcast tournament to relay external games. This endpoint accepts the same form data as the [web form](https://lichess.org/broadcast/new). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the broadcast tournament. Length must be between 3 and 80 characters.  Example: `Sinquefield Cup`  | [required] |
**description** | **String** | Short description of the broadcast tournament. Length must be between 3 and 400 characters.  Example: `An 11 round classical tournament featuring the 9 highest rated players in the world. Including Carlsen, Caruana, Ding, Aronian, Nakamura and more.`  | [required] |
**auto_leaderboard** | **bool** | Compute and display a simple leaderboard based on game results | [required] |
**markdown** | Option<**String**> | Optional long description of the broadcast. Markdown is supported. Length must be less than 20,000 characters. |  |
**tier** | Option<**i32**> | Optional, for Lichess admins only, use to feature on /broadcast.  * `3` for normal * `4` for high * `5` for best  |  |
**players** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_tour_get

> serde_json::Value broadcast_tour_get(slug, broadcast_tournament_id)
Get your broadcast tournament

Get information about a broadcast tournament. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The broadcast tournament slug. Only used for SEO, the slug can be safely replaced by `-`. Only the `broadcastTournamentId` is actually used. | [required] |
**broadcast_tournament_id** | **String** | The broadcast tournament ID (8 characters). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_tour_update

> crate::models::Ok broadcast_tour_update(broadcast_tournament_id, name, description, auto_leaderboard, markdown, tier, players)
Update your broadcast tournament

Update information about a broadcast tournament that you created. This endpoint accepts the same form data as the web form. All fields must be populated with data. Missing fields will override the broadcast with empty data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**broadcast_tournament_id** | **String** | The broadcast ID (8 characters). | [required] |
**name** | **String** | Name of the broadcast tournament. Length must be between 3 and 80 characters.  Example: `Sinquefield Cup`  | [required] |
**description** | **String** | Short description of the broadcast tournament. Length must be between 3 and 400 characters.  Example: `An 11 round classical tournament featuring the 9 highest rated players in the world. Including Carlsen, Caruana, Ding, Aronian, Nakamura and more.`  | [required] |
**auto_leaderboard** | **bool** | Compute and display a simple leaderboard based on game results | [required] |
**markdown** | Option<**String**> | Optional long description of the broadcast. Markdown is supported. Length must be less than 20,000 characters. |  |
**tier** | Option<**i32**> | Optional, for Lichess admins only, use to feature on /broadcast.  * `3` for normal * `4` for high * `5` for best  |  |
**players** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

