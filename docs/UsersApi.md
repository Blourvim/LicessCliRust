# \UsersApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_crosstable**](UsersApi.md#api_crosstable) | **GET** /api/crosstable/{user1}/{user2} | Get crosstable
[**api_player_autocomplete**](UsersApi.md#api_player_autocomplete) | **GET** /api/player/autocomplete | Autocomplete usernames
[**api_user**](UsersApi.md#api_user) | **GET** /api/user/{username} | Get user public data
[**api_user_activity**](UsersApi.md#api_user_activity) | **GET** /api/user/{username}/activity | Get user activity
[**api_user_perf**](UsersApi.md#api_user_perf) | **GET** /api/user/{username}/perf/{perf} | Get performance statistics of a user
[**api_user_rating_history**](UsersApi.md#api_user_rating_history) | **GET** /api/user/{username}/rating-history | Get rating history of a user
[**api_users**](UsersApi.md#api_users) | **POST** /api/users | Get users by ID
[**api_users_status**](UsersApi.md#api_users_status) | **GET** /api/users/status | Get real-time users status
[**player**](UsersApi.md#player) | **GET** /api/player | Get all top 10
[**player_top_nb_perf_type**](UsersApi.md#player_top_nb_perf_type) | **GET** /api/player/top/{nb}/{perfType} | Get one leaderboard
[**read_note**](UsersApi.md#read_note) | **GET** /api/user/{username}/note | Get notes for a user
[**streamer_live**](UsersApi.md#streamer_live) | **GET** /api/streamer/live | Get live streamers
[**write_note**](UsersApi.md#write_note) | **POST** /api/user/{username}/note | Add a note for a user



## api_crosstable

> serde_json::Value api_crosstable(user1, user2, matchup)
Get crosstable

Get total number of games, and current score, of any two users.  If the `matchup` flag is provided, and the users are currently playing, also gets the current match game number and scores. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user1** | **String** |  | [required] |
**user2** | **String** |  | [required] |
**matchup** | Option<**bool**> | Whether to get the current match data, if any |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_player_autocomplete

> crate::models::OneOfLessThanGreaterThan api_player_autocomplete(term, object, friend)
Autocomplete usernames

Provides autocompletion options for an incomplete username. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The beginning of a username | [required] |
**object** | Option<**bool**> | - `false` returns an array of usernames - `true` returns an object with matching users  |  |[default to false]
**friend** | Option<**bool**> | Returns followed players matching `term` if any, else returns other players. Requires [OAuth](#tag/OAuth).  |  |

### Return type

[**crate::models::OneOfLessThanGreaterThan**](oneOf<>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_user

> crate::models::UserExtended api_user(username, trophies)
Get user public data

Read public data of a user.  If the request is [authenticated with OAuth2](#section/Introduction/Authentication), then extra fields might be present in the response: `followable`, `following`, `blocking`, `followsYou`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**trophies** | Option<**bool**> | Include user trophies |  |[default to false]

### Return type

[**crate::models::UserExtended**](UserExtended.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_user_activity

> api_user_activity(username)
Get user activity

Read data to generate the activity feed of a user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_user_perf

> serde_json::Value api_user_perf(username, perf)
Get performance statistics of a user

Read performance statistics of a user, for a single performance. Similar to the [performance pages on the website](https://lichess.org/@/thibault/perf/bullet). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**perf** | [**PerfType**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_user_rating_history

> serde_json::Value api_user_rating_history(username)
Get rating history of a user

Read rating history of a user, for all perf types. There is at most one entry per day. Format of an entry is `[year, month, day, rating]`. `month` starts at zero (January). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_users

> Vec<crate::models::User> api_users(body)
Get users by ID

Get up to 300 users by their IDs. Users are returned in the same order as the IDs.  The method is `POST` to allow a longer list of IDs to be sent in the request body.  Please do not try to download all the Lichess users with this endpoint, or any other endpoint. An API is not a way to fully export a website. We do not provide a full download of the Lichess users.  This endpoint is limited to 8,000 users every 10 minutes, and 120,000 every day. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | User IDs separated by commas. | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_users_status

> Vec<crate::models::ApiUsersStatus200ResponseInner> api_users_status(ids, with_game_ids)
Get real-time users status

Read the `online`, `playing` and `streaming` flags of several users.  This API is very fast and cheap on lichess side. So you can call it quite often (like once every 5 seconds).  Use it to track players and know when they're connected on lichess and playing games. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | User IDs separated by commas. Up to 100 IDs. | [required] |
**with_game_ids** | Option<**bool**> | Also return the ID of the game being played, if any, for each player, in a `playingId` field. Defaults to `false` to preserve server resources.  |  |

### Return type

[**Vec<crate::models::ApiUsersStatus200ResponseInner>**](apiUsersStatus_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player

> serde_json::Value player()
Get all top 10

Get the top 10 players for each speed and variant.  See <https://lichess.org/player>. 

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


## player_top_nb_perf_type

> serde_json::Value player_top_nb_perf_type(nb, perf_type)
Get one leaderboard

Get the leaderboard for a single speed or variant (a.k.a. `perfType`). There is no leaderboard for correspondence or puzzles.  See <https://lichess.org/player/top/200/bullet>. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nb** | **i32** | How many users to fetch | [required] |
**perf_type** | **String** | The speed or variant | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.lichess.v3+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_note

> Vec<crate::models::UserNote> read_note(username)
Get notes for a user

Get the private notes that you have added for a user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**Vec<crate::models::UserNote>**](UserNote.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## streamer_live

> Vec<crate::models::LightUser> streamer_live()
Get live streamers

Get basic info about currently streaming users.  This API is very fast and cheap on lichess side. So you can call it quite often (like once every 5 seconds). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LightUser>**](LightUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_note

> crate::models::Ok write_note(username, text)
Add a note for a user

Add a private note available only to you about this account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**text** | **String** | The contents of the note | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

