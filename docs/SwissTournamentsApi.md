# \SwissTournamentsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_swiss_join**](SwissTournamentsApi.md#api_swiss_join) | **POST** /api/swiss/{id}/join | Join a Swiss tournament
[**api_swiss_new**](SwissTournamentsApi.md#api_swiss_new) | **POST** /api/swiss/new/{teamId} | Create a new Swiss tournament
[**api_swiss_schedule_next_round**](SwissTournamentsApi.md#api_swiss_schedule_next_round) | **POST** /api/swiss/{id}/schedule-next-round | Manually schedule the next round
[**api_swiss_terminate**](SwissTournamentsApi.md#api_swiss_terminate) | **POST** /api/swiss/{id}/terminate | Terminate a Swiss tournament
[**api_swiss_update**](SwissTournamentsApi.md#api_swiss_update) | **POST** /api/swiss/{id}/edit | Update a Swiss tournament
[**api_swiss_withdraw**](SwissTournamentsApi.md#api_swiss_withdraw) | **POST** /api/swiss/{id}/withdraw | Pause or leave a swiss tournament
[**api_team_swiss**](SwissTournamentsApi.md#api_team_swiss) | **GET** /api/team/{teamId}/swiss | Get team swiss tournaments
[**games_by_swiss**](SwissTournamentsApi.md#games_by_swiss) | **GET** /api/swiss/{id}/games | Export games of a Swiss tournament
[**results_by_swiss**](SwissTournamentsApi.md#results_by_swiss) | **GET** /api/swiss/{id}/results | Get results of a swiss tournament
[**swiss**](SwissTournamentsApi.md#swiss) | **GET** /api/swiss/{id} | Get info about a Swiss tournament
[**swiss_trf**](SwissTournamentsApi.md#swiss_trf) | **GET** /swiss/{id}.trf | Export TRF of a Swiss tournament



## api_swiss_join

> crate::models::Ok api_swiss_join(id, password)
Join a Swiss tournament

Join a Swiss tournament, possibly with a password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**password** | Option<**String**> | The tournament password, if one is required |  |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_swiss_new

> serde_json::Value api_swiss_new(team_id, clock_period_limit, clock_period_increment, nb_rounds, name, starts_at, round_interval, variant, position, description, rated, password, forbidden_pairings, manual_pairings, chat_for, conditions_period_min_rating_period_rating, conditions_period_max_rating_period_rating, conditions_period_nb_rated_game_period_nb, conditions_period_play_your_games, conditions_period_allow_list)
Create a new Swiss tournament

Create a Swiss tournament for your team.  This endpoint mirrors the Swiss tournament form from your team pagee.  You can create up to 12 tournaments per day.  Additional restrictions:   - clock.limit + clock.increment > 0   - 15s and 0+1 variant tournaments cannot be rated 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | ID of the team | [required] |
**clock_period_limit** | **f32** | Clock initial time in seconds | [required] |
**clock_period_increment** | **i32** | Clock increment in seconds | [required] |
**nb_rounds** | **i32** | Maximum number of rounds to play | [required] |
**name** | Option<**String**> | The tournament name. Leave empty to get a random Grandmaster name |  |
**starts_at** | Option<**i32**> | Timestamp in milliseconds to start the tournament at a given date and time. By default, it starts 10 minutes after creation. |  |
**round_interval** | Option<**i32**> | How long to wait between each round, in seconds.  Set to 99999999 to manually schedule each round from the tournament UI.  If empty or -1, a sensible value is picked automatically.  |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**position** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**description** | Option<**String**> | Anything you want to tell players about the tournament |  |
**rated** | Option<**bool**> | Games are rated and impact players ratings |  |[default to true]
**password** | Option<**String**> | Make the tournament private and restrict access with a password. |  |
**forbidden_pairings** | Option<**String**> | Usernames of players that must not play together.  Two usernames per line, separated by a space.  |  |
**manual_pairings** | Option<**String**> | Manual pairings for the next round.  Two usernames per line, separated by a space. Example: ``` PlayerA PlayerB PlayerC PlayerD ```  To give a bye (1 point) to a player instead of a pairing, add a line like so: ``` PlayerE 1 ```  Missing players will be considered absent and get zero points.  |  |
**chat_for** | Option<**f32**> | Who can read and write in the chat. - 0  = No-one - 10 = Only team leaders - 20 = Only team members - 30 = All Lichess players  |  |[default to 20]
**conditions_period_min_rating_period_rating** | Option<**i32**> | Minimum rating to join. Leave empty to let everyone join the tournament. |  |
**conditions_period_max_rating_period_rating** | Option<**i32**> | Maximum rating to join. Based on best rating reached in the last 7 days. Leave empty to let everyone join the tournament. |  |
**conditions_period_nb_rated_game_period_nb** | Option<**i32**> | Minimum number of rated games required to join. |  |
**conditions_period_play_your_games** | Option<**bool**> | Only let players join if they have played their last swiss game. If they failed to show up in a recent swiss event, they won't be able to enter yours. This results in a better swiss experience for the players who actually show up.  |  |[default to false]
**conditions_period_allow_list** | Option<**String**> | Predefined list of usernames that are allowed to join, separated by commas. If this list is non-empty, then usernames absent from this list will be forbidden to join. Adding `%titled` to the list additionally allows any titled player to join. Example: `thibault,german11,%titled`  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_swiss_schedule_next_round

> api_swiss_schedule_next_round(id, date)
Manually schedule the next round

Manually schedule the next round date and time of a Swiss tournament.  This sets the `roundInterval` field to `99999999`, i.e. manual scheduling.  All further rounds will need to be manually scheduled, unless the `roundInterval` field is changed back to automatic scheduling. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**date** | Option<**i32**> | Timestamp in milliseconds to start the next round at a given date and time. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_swiss_terminate

> crate::models::Ok api_swiss_terminate(id)
Terminate a Swiss tournament

Terminate a Swiss tournament 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Swiss tournament ID. | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_swiss_update

> serde_json::Value api_swiss_update(id, clock_period_limit, clock_period_increment, nb_rounds, name, starts_at, round_interval, variant, description, rated, password, forbidden_pairings, manual_pairings, chat_for, conditions_period_min_rating_period_rating, conditions_period_max_rating_period_rating, conditions_period_nb_rated_game_period_nb, conditions_period_play_your_games, conditions_period_allow_list)
Update a Swiss tournament

Update a Swiss tournament.  Be mindful not to make important changes to ongoing tournaments.  Additional restrictions:   - clock.limit + clock.increment > 0   - 15s and 0+1 variant tournaments cannot be rated 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**clock_period_limit** | **f32** | Clock initial time in seconds | [required] |
**clock_period_increment** | **i32** | Clock increment in seconds | [required] |
**nb_rounds** | **i32** | Maximum number of rounds to play | [required] |
**name** | Option<**String**> | The tournament name. Leave empty to get a random Grandmaster name |  |
**starts_at** | Option<**i32**> | Timestamp in milliseconds to start the tournament at a given date and time. By default, it starts 10 minutes after creation. |  |
**round_interval** | Option<**i32**> | How long to wait between each round, in seconds.  Set to 99999999 to manually schedule each round from the tournament UI, or [with the API](#tag/Swiss-tournaments/operation/apiSwissScheduleNextRound).  If empty or -1, a sensible value is picked automatically.  |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**description** | Option<**String**> | Anything you want to tell players about the tournament |  |
**rated** | Option<**bool**> | Games are rated and impact players ratings |  |[default to true]
**password** | Option<**String**> | Make the tournament private and restrict access with a password. |  |
**forbidden_pairings** | Option<**String**> | Usernames of players that must not play together.  Two usernames per line, separated by a space.  |  |
**manual_pairings** | Option<**String**> | Manual pairings for the next round.  Two usernames per line, separated by a space. Present players without a valid pairing will be given a bye, which is worth 1 point. Forfeited players will get 0 points.  |  |
**chat_for** | Option<**f32**> | Who can read and write in the chat. - 0  = No-one - 10 = Only team leaders - 20 = Only team members - 30 = All Lichess players  |  |[default to 20]
**conditions_period_min_rating_period_rating** | Option<**i32**> | Minimum rating to join. Leave empty to let everyone join the tournament. |  |
**conditions_period_max_rating_period_rating** | Option<**i32**> | Maximum rating to join. Based on best rating reached in the last 7 days. Leave empty to let everyone join the tournament. |  |
**conditions_period_nb_rated_game_period_nb** | Option<**i32**> | Minimum number of rated games required to join. |  |
**conditions_period_play_your_games** | Option<**bool**> | Only let players join if they have played their last swiss game. If they failed to show up in a recent swiss event, they won't be able to enter yours. This results in a better swiss experience for the players who actually show up.  |  |[default to false]
**conditions_period_allow_list** | Option<**String**> | Predefined list of usernames that are allowed to join, separated by commas. If this list is non-empty, then usernames absent from this list will be forbidden to join. Adding `%titled` to the list additionally allows any titled player to join. Example: `thibault,german11,%titled`  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_swiss_withdraw

> crate::models::Ok api_swiss_withdraw(id)
Pause or leave a swiss tournament

Leave a future Swiss tournament, or take a break on an ongoing Swiss tournament. It's possible to join again later. Points are preserved. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## games_by_swiss

> serde_json::Value games_by_swiss(id, player, moves, pgn_in_json, tags, clocks, evals, accuracy, opening)
Export games of a Swiss tournament

Download games of a swiss tournament in PGN or [ndjson](#section/Introduction/Streaming-with-ND-JSON) format.  Games are sorted by reverse chronological order (last round first).  The game stream is throttled, depending on who is making the request:   - Anonymous request: 20 games per second   - [OAuth2 authenticated](#section/Introduction/Authentication) request: 30 games per second 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**player** | Option<**String**> | Only the games played by a given player |  |
**moves** | Option<**bool**> | Include the PGN moves. |  |[default to true]
**pgn_in_json** | Option<**bool**> | Include the full PGN within the JSON response, in a `pgn` field. |  |[default to false]
**tags** | Option<**bool**> | Include the PGN tags. |  |[default to true]
**clocks** | Option<**bool**> | Include clock status when available.  Either as PGN comments: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  Or in a `clocks` JSON field, as centisecond integers, depending on the response type.  |  |[default to false]
**evals** | Option<**bool**> | Include analysis evaluations and comments, when available.  Either as PGN comments: `12. Bxf6 { [%eval 0.23] } a3 { [%eval -1.09] }`  Or in an `analysis` JSON field, depending on the response type.  |  |[default to false]
**accuracy** | Option<**bool**> | Include [accuracy percent](https://lichess.org/page/accuracy) of each player, when available.  |  |[default to false]
**opening** | Option<**bool**> | Include the opening name.  Example: `[Opening \"King's Gambit Accepted, King's Knight Gambit\"]`  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn, application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## results_by_swiss

> serde_json::Value results_by_swiss(id, nb)
Get results of a swiss tournament

Players of a swiss tournament, with their score and performance, sorted by rank (best first).  Players are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON).  If called on an ongoing tournament, results can be inconsistent due to ranking changes while the players are being streamed. Use on finished tournaments for guaranteed consistency. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**nb** | Option<**i32**> | Max number of players to fetch |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swiss

> serde_json::Value swiss(id)
Get info about a Swiss tournament

Get detailed info about a Swiss tournament. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Swiss tournament ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swiss_trf

> String swiss_trf(id)
Export TRF of a Swiss tournament

Download a tournament in the Tournament Report File format, the FIDE standard.  Documentation: <https://www.fide.com/FIDE/handbook/C04Annex2_TRF16.pdf>  Example: <https://lichess.org/swiss/j8rtJ5GL.trf> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

