# \TeamsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_team_arena**](TeamsApi.md#api_team_arena) | **GET** /api/team/{teamId}/arena | Get team Arena tournaments
[**api_team_swiss**](TeamsApi.md#api_team_swiss) | **GET** /api/team/{teamId}/swiss | Get team swiss tournaments
[**team_all**](TeamsApi.md#team_all) | **GET** /api/team/all | Get popular teams
[**team_id_join**](TeamsApi.md#team_id_join) | **POST** /team/{teamId}/join | Join a team
[**team_id_kick_user_id**](TeamsApi.md#team_id_kick_user_id) | **POST** /api/team/{teamId}/kick/{userId} | Kick a user from your team
[**team_id_pm_all**](TeamsApi.md#team_id_pm_all) | **POST** /team/{teamId}/pm-all | Message all members
[**team_id_quit**](TeamsApi.md#team_id_quit) | **POST** /team/{teamId}/quit | Leave a team
[**team_id_users**](TeamsApi.md#team_id_users) | **GET** /api/team/{teamId}/users | Get members of a team
[**team_of_username**](TeamsApi.md#team_of_username) | **GET** /api/team/of/{username} | Teams of a player
[**team_request_accept**](TeamsApi.md#team_request_accept) | **POST** /api/team/{teamId}/request/{userId}/accept | Accept join request
[**team_request_decline**](TeamsApi.md#team_request_decline) | **POST** /api/team/{teamId}/request/{userId}/decline | Decline join request
[**team_requests**](TeamsApi.md#team_requests) | **GET** /api/team/{teamId}/requests | Get join requests
[**team_search**](TeamsApi.md#team_search) | **GET** /api/team/search | Search teams
[**team_show**](TeamsApi.md#team_show) | **GET** /api/team/{teamId} | Get a single team



## api_team_arena

> crate::models::ArenaTournament api_team_arena(team_id, max)
Get team Arena tournaments

Get all Arena tournaments relevant to a team.  Tournaments are sorted by reverse chronological order of start date (last starting first).  Tournaments are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | ID of the team | [required] |
**max** | Option<**i32**> | How many tournaments to download. |  |[default to 100]

### Return type

[**crate::models::ArenaTournament**](ArenaTournament.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_team_swiss

> serde_json::Value api_team_swiss(team_id, max)
Get team swiss tournaments

Get all swiss tournaments of a team.  Tournaments are sorted by reverse chronological order of start date (last starting first).  Tournaments are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**max** | Option<**i32**> | How many tournaments to download. |  |[default to 100]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/nd-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_all

> crate::models::TeamPaginatorJson team_all(page)
Get popular teams

Paginator of the most popular teams. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f32**> |  |  |[default to 1]

### Return type

[**crate::models::TeamPaginatorJson**](TeamPaginatorJson.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_id_join

> crate::models::Ok team_id_join(team_id, message, password)
Join a team

Join a team. If the team requires a password but the `password` field is incorrect, then the call fails with `403 Forbidden`. Similarly, if the team join policy requires a confirmation but the `message` parameter is not given, then the call fails with `403 Forbidden`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**message** | Option<**String**> | Optional request message, if the team requires one. |  |
**password** | Option<**String**> | Optional password, if the team requires one. |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_id_kick_user_id

> crate::models::Ok team_id_kick_user_id(team_id, user_id)
Kick a user from your team

Kick a member out of one of your teams. - <https://lichess.org/team> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_id_pm_all

> crate::models::Ok team_id_pm_all(team_id, message)
Message all members

Send a private message to all members of a team. You must be a team leader with the \"Messages\" permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**message** | Option<**String**> | The message to send to all your team members. |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_id_quit

> crate::models::Ok team_id_quit(team_id)
Leave a team

Leave a team. - <https://lichess.org/team> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_id_users

> crate::models::UserExtended team_id_users(team_id)
Get members of a team

Members are sorted by reverse chronological order of joining the team (most recent first). OAuth is only required if the list of members is private.  Members are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**crate::models::UserExtended**](UserExtended.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_of_username

> Vec<crate::models::Team> team_of_username(username)
Teams of a player

All the teams a player is a member of. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_request_accept

> crate::models::Ok team_request_accept(team_id, user_id)
Accept join request

Accept someone's request to join your team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_request_decline

> crate::models::Ok team_request_decline(team_id, user_id)
Decline join request

Decline someone's request to join your team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_requests

> Vec<crate::models::TeamRequestWithUser> team_requests(team_id, declined)
Get join requests

Get pending join requests of your team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**declined** | Option<**bool**> | Get the declined join requests |  |[default to false]

### Return type

[**Vec<crate::models::TeamRequestWithUser>**](TeamRequestWithUser.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_search

> crate::models::TeamPaginatorJson team_search(text, page)
Search teams

Paginator of team search results for a keyword. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | Option<**String**> |  |  |
**page** | Option<**f32**> |  |  |[default to 1]

### Return type

[**crate::models::TeamPaginatorJson**](TeamPaginatorJson.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_show

> crate::models::Team team_show(team_id)
Get a single team

Public info about a team. Includes the list of publicly visible leaders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

