# \ArenaTournamentsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_team_arena**](ArenaTournamentsApi.md#api_team_arena) | **GET** /api/team/{teamId}/arena | Get team Arena tournaments
[**api_tournament**](ArenaTournamentsApi.md#api_tournament) | **GET** /api/tournament | Get current tournaments
[**api_tournament_join**](ArenaTournamentsApi.md#api_tournament_join) | **POST** /api/tournament/{id}/join | Join an Arena tournament
[**api_tournament_post**](ArenaTournamentsApi.md#api_tournament_post) | **POST** /api/tournament | Create a new Arena tournament
[**api_tournament_team_battle_post**](ArenaTournamentsApi.md#api_tournament_team_battle_post) | **POST** /api/tournament/team-battle/{id} | Update a team battle
[**api_tournament_terminate**](ArenaTournamentsApi.md#api_tournament_terminate) | **POST** /api/tournament/{id}/terminate | Terminate an Arena tournament
[**api_tournament_update**](ArenaTournamentsApi.md#api_tournament_update) | **POST** /api/tournament/{id} | Update an Arena tournament
[**api_tournament_withdraw**](ArenaTournamentsApi.md#api_tournament_withdraw) | **POST** /api/tournament/{id}/withdraw | Pause or leave an Arena tournament
[**api_user_name_tournament_created**](ArenaTournamentsApi.md#api_user_name_tournament_created) | **GET** /api/user/{username}/tournament/created | Get tournaments created by a user
[**games_by_tournament**](ArenaTournamentsApi.md#games_by_tournament) | **GET** /api/tournament/{id}/games | Export games of an Arena tournament
[**results_by_tournament**](ArenaTournamentsApi.md#results_by_tournament) | **GET** /api/tournament/{id}/results | Get results of an Arena tournament
[**teams_by_tournament**](ArenaTournamentsApi.md#teams_by_tournament) | **GET** /api/tournament/{id}/teams | Get team standing of a team battle
[**tournament**](ArenaTournamentsApi.md#tournament) | **GET** /api/tournament/{id} | Get info about an Arena tournament



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


## api_tournament

> crate::models::ArenaTournaments api_tournament()
Get current tournaments

Get recently finished, ongoing, and upcoming tournaments.  This API is used to display the [Lichess tournament schedule](https://lichess.org/tournament). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ArenaTournaments**](ArenaTournaments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tournament_join

> crate::models::Ok api_tournament_join(id, password, team, pair_me_asap)
Join an Arena tournament

Join an Arena tournament, possibly with a password and/or a team. Also unpauses if you had previously [paused](#operation/apiTournamentWithdraw) the tournament. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**password** | Option<**String**> | The tournament password, if one is required. Can also be a [user-specific entry code](https://github.com/lichess-org/api/tree/master/example/tournament-entry-code) generated and shared by the organizer.  |  |
**team** | Option<**String**> | The team to join the tournament with, for team battle tournaments |  |
**pair_me_asap** | Option<**bool**> | If the tournament is started, attempt to pair the user, even if they are not connected to the tournament page. This expires after one minute, to avoid pairing a user who is long gone. You may call \\\"join\\\" again to extend the waiting.  |  |[default to false]

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tournament_post

> serde_json::Value api_tournament_post(clock_time, clock_increment, minutes, name, wait_minutes, start_date, variant, rated, position, berserkable, streakable, has_chat, description, password, team_battle_by_team, conditions_period_team_member_period_team_id, conditions_period_min_rating_period_rating, conditions_period_max_rating_period_rating, conditions_period_nb_rated_game_period_nb, conditions_period_allow_list)
Create a new Arena tournament

Create a public or private Arena tournament.  This endpoint mirrors the form on <https://lichess.org/tournament/new>.  You can create up to 12 public tournaments per day, or 24 private tournaments.  A team battle can be created by specifying the `teamBattleByTeam` argument.  Additional restrictions:   - clockTime + clockIncrement > 0   - 15s and 0+1 variant tournaments cannot be rated   - Clock time in comparison to tournament length must be reasonable: 3 <= (minutes * 60) / (96 * clockTime + 48 * clockIncrement + 15) <= 150 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clock_time** | **f32** | Clock initial time in minutes | [required] |
**clock_increment** | **i32** | Clock increment in seconds | [required] |
**minutes** | **i32** | How long the tournament lasts, in minutes | [required] |
**name** | Option<**String**> | The tournament name. Leave empty to get a random Grandmaster name |  |
**wait_minutes** | Option<**i32**> | How long to wait before starting the tournament, from now, in minutes |  |[default to 5]
**start_date** | Option<**i32**> | Timestamp (in milliseconds) to start the tournament at a given date and time. Overrides the `waitMinutes` setting |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**rated** | Option<**bool**> | Games are rated and impact players ratings |  |[default to true]
**position** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**berserkable** | Option<**bool**> | Whether the players can use berserk. Only allowed if clockIncrement <= clockTime * 2 |  |[default to true]
**streakable** | Option<**bool**> | After 2 wins, consecutive wins grant 4 points instead of 2. |  |[default to true]
**has_chat** | Option<**bool**> | Whether the players can discuss in a chat |  |[default to true]
**description** | Option<**String**> | Anything you want to tell players about the tournament |  |
**password** | Option<**String**> | Make the tournament private, and restrict access with a password. You can also [generate user-specific entry codes](https://github.com/lichess-org/api/tree/master/example/tournament-entry-code) based on this password.  |  |
**team_battle_by_team** | Option<**String**> | Set the ID of a team you lead to create a team battle. The other teams can be added using the [team battle edit endpoint](#operation/apiTournamentTeamBattlePost).  |  |
**conditions_period_team_member_period_team_id** | Option<**String**> | Restrict entry to members of a team.  The teamId is the last part of a team URL, e.g. `https://lichess.org/team/coders` has teamId = `coders`.  Leave empty to let everyone join the tournament.  Do not use this to create team battles, use `teamBattleByTeam` instead.  |  |
**conditions_period_min_rating_period_rating** | Option<**i32**> | Minimum rating to join. Leave empty to let everyone join the tournament. |  |
**conditions_period_max_rating_period_rating** | Option<**i32**> | Maximum rating to join. Based on best rating reached in the last 7 days. Leave empty to let everyone join the tournament. |  |
**conditions_period_nb_rated_game_period_nb** | Option<**i32**> | Minimum number of rated games required to join. |  |
**conditions_period_allow_list** | Option<**String**> | Predefined list of usernames that are allowed to join, separated by commas. If this list is non-empty, then usernames absent from this list will be forbidden to join. Adding `%titled` to the list additionally allows any titled player to join. Example: `thibault,german11,%titled`  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tournament_team_battle_post

> serde_json::Value api_tournament_team_battle_post(id, teams, nb_leaders)
Update a team battle

Set the teams and number of leaders of a team battle.  To update the other attributes of a team battle, use the [tournament update endpoint](#operation/apiTournamentUpdate). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID (8 characters).. | [required] |
**teams** | **String** | All team IDs of the team battle, separated by commas. Make sure to always send the full list. Teams that are not in the list will be removed from the team battle.  Example: `coders,zhigalko_sergei-fan-club,hhSwTKZv`  | [required] |
**nb_leaders** | **i32** | Number team leaders per team. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tournament_terminate

> crate::models::Ok api_tournament_terminate(id)
Terminate an Arena tournament

Terminate an Arena tournament 

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


## api_tournament_update

> serde_json::Value api_tournament_update(id, clock_time, clock_increment, minutes, name, wait_minutes, start_date, variant, rated, position, berserkable, streakable, has_chat, description, password, conditions_period_min_rating_period_rating, conditions_period_max_rating_period_rating, conditions_period_nb_rated_game_period_nb, conditions_period_allow_list)
Update an Arena tournament

Update an Arena tournament.  Be mindful not to make important changes to ongoing tournaments.  Can be used to update a team battle.  Additional restrictions:   - clockTime + clockIncrement > 0   - 15s and 0+1 variant tournaments cannot be rated   - Clock time in comparison to tournament length must be reasonable: 3 <= (minutes * 60) / (96 * clockTime + 48 * clockIncrement + 15) <= 150 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**clock_time** | **f32** | Clock initial time in minutes | [required] |
**clock_increment** | **i32** | Clock increment in seconds | [required] |
**minutes** | **i32** | How long the tournament lasts, in minutes | [required] |
**name** | Option<**String**> | The tournament name. Leave empty to get a random Grandmaster name |  |
**wait_minutes** | Option<**i32**> | How long to wait before starting the tournament, from now, in minutes |  |[default to 5]
**start_date** | Option<**i32**> | Timestamp (in milliseconds) to start the tournament at a given date and time. Overrides the `waitMinutes` setting |  |
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |
**rated** | Option<**bool**> | Games are rated and impact players ratings |  |[default to true]
**position** | Option<**String**> | Custom initial position (in FEN). Variant must be standard, fromPosition, or chess960 (if a valid 960 starting position), and the game cannot be rated. |  |
**berserkable** | Option<**bool**> | Whether the players can use berserk. Only allowed if clockIncrement <= clockTime * 2 |  |[default to true]
**streakable** | Option<**bool**> | After 2 wins, consecutive wins grant 4 points instead of 2. |  |[default to true]
**has_chat** | Option<**bool**> | Whether the players can discuss in a chat |  |[default to true]
**description** | Option<**String**> | Anything you want to tell players about the tournament |  |
**password** | Option<**String**> | Make the tournament private, and restrict access with a password |  |
**conditions_period_min_rating_period_rating** | Option<**i32**> | Minimum rating to join. Leave empty to let everyone join the tournament. |  |
**conditions_period_max_rating_period_rating** | Option<**i32**> | Maximum rating to join. Based on best rating reached in the last 7 days. Leave empty to let everyone join the tournament. |  |
**conditions_period_nb_rated_game_period_nb** | Option<**i32**> | Minimum number of rated games required to join. |  |
**conditions_period_allow_list** | Option<**String**> | Predefined list of usernames that are allowed to join, separated by commas. If this list is non-empty, then usernames absent from this list will be forbidden to join. Adding `%titled` to the list additionally allows any titled player to join. Example: `thibault,german11,%titled`  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tournament_withdraw

> crate::models::Ok api_tournament_withdraw(id)
Pause or leave an Arena tournament

Leave a future Arena tournament, or take a break on an ongoing Arena tournament. It's possible to join again later. Points and streaks are preserved. 

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


## api_user_name_tournament_created

> crate::models::ArenaTournament api_user_name_tournament_created(username, status)
Get tournaments created by a user

Get all tournaments created by a given user.  Tournaments are sorted by reverse chronological order of start date (last starting first).  Tournaments are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The user whose created tournaments to fetch | [required] |
**status** | Option<**i32**> | Include tournaments in the given status: \"Created\" (10), \"Started\" (20), \"Finished\" (30)  You can add this parameter more than once to include tournaments in different statuses.  Example: `?status=10&status=20`  |  |

### Return type

[**crate::models::ArenaTournament**](ArenaTournament.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_tournament

> serde_json::Value games_by_tournament(id, player, moves, pgn_in_json, tags, clocks, evals, accuracy, opening)
Export games of an Arena tournament

Download games of a tournament in PGN or [ndjson](#section/Introduction/Streaming-with-ND-JSON) format.  Games are sorted by reverse chronological order (most recent first).  The game stream is throttled, depending on who is making the request:   - Anonymous request: 20 games per second   - [OAuth2 authenticated](#section/Introduction/Authentication) request: 30 games per second 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**player** | Option<**String**> | Only games of a particular player. Leave empty to fetch games of all players. |  |
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


## results_by_tournament

> serde_json::Value results_by_tournament(id, nb, sheet)
Get results of an Arena tournament

Players of an Arena tournament, with their score and performance, sorted by rank (best first).  **Players are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON)**, i.e. one JSON object per line.  If called on an ongoing tournament, results can be inconsistent due to ranking changes while the players are being streamed. Use on finished tournaments for guaranteed consistency. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**nb** | Option<**i32**> | Max number of players to fetch |  |
**sheet** | Option<**bool**> | Add a `sheet` field to the player document. It's an expensive server computation that slows down the stream.  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_by_tournament

> serde_json::Value teams_by_tournament(id)
Get team standing of a team battle

Teams of a team battle tournament, with top players, sorted by rank (best first). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tournament

> serde_json::Value tournament(id, page)
Get info about an Arena tournament

Get detailed info about recently finished, current, or upcoming tournament's duels, player standings, and other info. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The tournament ID. | [required] |
**page** | Option<**f32**> | Specify which page of player standings to view. |  |[default to 1]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

