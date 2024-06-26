/*
 * Lichess.org API reference
 *
 * # Introduction Welcome to the reference for the Lichess API! Lichess is free/libre, open-source chess server powered by volunteers and donations. - Get help in the [Lichess Discord channel](https://discord.gg/lichess) - API demo app with OAuth2 login and gameplay: [source](https://github.com/lichess-org/api-demo) / [demo](https://lichess-org.github.io/api-demo/) - API UI app with OAuth2 login and endpoint forms: [source](https://github.com/lichess-org/api-ui) / [website](https://lichess.org/api/ui) - [Contribute to this documentation on Github](https://github.com/lichess-org/api) - Check out [Lichess widgets to embed in your website](https://lichess.org/developers) - [Download all Lichess rated games](https://database.lichess.org/) - [Download all Lichess puzzles with themes, ratings and votes](https://database.lichess.org/#puzzles) - [Download all evaluated positions](https://database.lichess.org/#evals)  ## Endpoint All requests go to `https://lichess.org` (unless otherwise specified).  ## Clients - [Python general API](https://github.com/lichess-org/berserk) - [MicroPython general API](https://github.com/mkomon/uberserk) - [Python general API - async](https://pypi.org/project/async-lichess-sdk) - [Python Lichess Bot](https://github.com/lichess-bot-devs/lichess-bot) - [Python Board API for Certabo](https://github.com/haklein/certabo-lichess) - [Java general API](https://github.com/tors42/chariot) - [JavaScript & TypeScript general API](https://github.com/devjiwonchoi/equine)  ## Rate limiting All requests are rate limited using various strategies, to ensure the API remains responsive for everyone. Only make one request at a time. If you receive an HTTP response with a [429 status](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#429), please wait a full minute before resuming API usage.  ## Streaming with ND-JSON Some API endpoints stream their responses as [Newline Delimited JSON a.k.a. **nd-json**](http://ndjson.org/), with one JSON object per line.  Here's a [JavaScript utility function](https://gist.github.com/ornicar/a097406810939cf7be1df8ea30e94f3e) to help reading NDJSON streamed responses.  ## Authentication ### Which authentication method is right for me? [Read about the Lichess API authentication methods and code examples](https://github.com/lichess-org/api/blob/master/example/README.md)  ### Personal Access Token Personal API access tokens allow you to quickly interact with Lichess API without going through an OAuth flow. - [Generate a personal access token](https://lichess.org/account/oauth/token) - `curl https://lichess.org/api/account -H \"Authorization: Bearer {token}\"` - [NodeJS example](https://github.com/lichess-org/api/tree/master/example/oauth-personal-token)  ### Authorization Code Flow with PKCE The authorization code flow with PKCE allows your users to **login with Lichess**. Lichess supports unregistered and public clients (no client authentication, choose any unique client id). The only accepted code challenge method is `S256`. Access tokens are long-lived (expect one year), unless they are revoked. Refresh tokens are not supported.  See the [documentation for the OAuth endpoints](#tag/OAuth) or the [PKCE RFC](https://datatracker.ietf.org/doc/html/rfc7636#section-4) for a precise protocol description.  - [Demo app](https://lichess-org.github.io/api-demo/) - [Minimal client-side example](https://github.com/lichess-org/api/tree/master/example/oauth-app) - [Flask/Python example](https://github.com/lakinwecker/lichess-oauth-flask) - [Java example](https://github.com/tors42/lichess-oauth-pkce-app) - [NodeJS Passport strategy to login with Lichess OAuth2](https://www.npmjs.com/package/passport-lichess)  #### Real life examples - [PyChess](https://github.com/gbtami/pychess-variants) ([source code](https://github.com/gbtami/pychess-variants)) - [Lichess4545](https://www.lichess4545.com/) ([source code](https://github.com/cyanfish/heltour)) - [English Chess Federation](https://ecf.octoknight.com/) - [Rotherham Online Chess](https://rotherhamonlinechess.azurewebsites.net/tournaments)  ### Token format Access tokens and authorization codes match `^[A-Za-z0-9_]+$`. The length of tokens can be increased without notice. Make sure your application can handle at least 512 characters. By convention tokens have a recognizable prefix, but do not rely on this. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: contact@lichess.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models::VariantKey};
use super::{Error, configuration};


/// struct for typed errors of method [`opening_explorer_lichess`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpeningExplorerLichessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`opening_explorer_master`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpeningExplorerMasterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`opening_explorer_master_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpeningExplorerMasterGameError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`opening_explorer_player`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpeningExplorerPlayerError {
    UnknownValue(serde_json::Value),
}


/// **Endpoint: <https://explorer.lichess.ovh/lichess>**  Games sampled from all Lichess players.  Example: `curl https://explorer.lichess.ovh/lichess?variant=standard&speeds=blitz,rapid,classical&ratings=2200,2500&fen=rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201` 
pub async fn opening_explorer_lichess(configuration: &configuration::Configuration, variant: Option<VariantKey>, fen: Option<&str>, play: Option<&str>, speeds: Option<Vec<crate::models::Speed>>, ratings: Option<Vec<f32>>, since: Option<&str>, until: Option<&str>, moves: Option<f32>, top_games: Option<f32>, recent_games: Option<f32>, history: Option<bool>) -> Result<serde_json::Value, Error<OpeningExplorerLichessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lichess", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = variant {
        local_var_req_builder = local_var_req_builder.query(&[("variant", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fen {
        local_var_req_builder = local_var_req_builder.query(&[("fen", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = play {
        local_var_req_builder = local_var_req_builder.query(&[("play", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = speeds {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("speeds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("speeds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = ratings {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("ratings".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("ratings", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = until {
        local_var_req_builder = local_var_req_builder.query(&[("until", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = moves {
        local_var_req_builder = local_var_req_builder.query(&[("moves", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = top_games {
        local_var_req_builder = local_var_req_builder.query(&[("topGames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recent_games {
        local_var_req_builder = local_var_req_builder.query(&[("recentGames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = history {
        local_var_req_builder = local_var_req_builder.query(&[("history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpeningExplorerLichessError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **Endpoint: <https://explorer.lichess.ovh/masters>**  Example: `curl https://explorer.lichess.ovh/masters?play=d2d4,d7d5,c2c4,c7c6,c4d5` 
pub async fn opening_explorer_master(configuration: &configuration::Configuration, fen: Option<&str>, play: Option<&str>, since: Option<f32>, until: Option<f32>, moves: Option<f32>, top_games: Option<f32>) -> Result<serde_json::Value, Error<OpeningExplorerMasterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/masters", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fen {
        local_var_req_builder = local_var_req_builder.query(&[("fen", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = play {
        local_var_req_builder = local_var_req_builder.query(&[("play", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = until {
        local_var_req_builder = local_var_req_builder.query(&[("until", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = moves {
        local_var_req_builder = local_var_req_builder.query(&[("moves", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = top_games {
        local_var_req_builder = local_var_req_builder.query(&[("topGames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpeningExplorerMasterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **Endpoint: `https://explorer.lichess.ovh/masters/pgn/{gameId}`**  Example: `curl https://explorer.lichess.ovh/masters/pgn/aAbqI4ey` 
pub async fn opening_explorer_master_game(configuration: &configuration::Configuration, game_id: &str) -> Result<serde_json::Value, Error<OpeningExplorerMasterGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/master/pgn/{gameId}", local_var_configuration.base_path, gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpeningExplorerMasterGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **Endpoint: <https://explorer.lichess.ovh/player>**  Games of a Lichess player.  Responds with a stream of [newline delimited JSON](#section/Introduction/Streaming-with-ND-JSON). Will start indexing on demand, immediately respond with the current results, and stream more updates until indexing is complete. The stream is throttled and deduplicated. Empty lines may be sent to avoid timeouts.  Will index new games at most once per minute, and revisit previously ongoing games at most once every day.  Example: `curl https://explorer.lichess.ovh/player?player=revoof&color=white&play=d2d4,d7d5&recentGames=1` 
pub async fn opening_explorer_player(configuration: &configuration::Configuration, player: Option<&str>, variant: Option<VariantKey>, fen: Option<&str>, play: Option<&str>, speeds: Option<Vec<crate::models::Speed>>, modes: Option<Vec<String>>, since: Option<&str>, until: Option<&str>, moves: Option<f32>, recent_games: Option<f32>) -> Result<serde_json::Value, Error<OpeningExplorerPlayerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/player", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = player {
        local_var_req_builder = local_var_req_builder.query(&[("player", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = variant {
        local_var_req_builder = local_var_req_builder.query(&[("variant", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fen {
        local_var_req_builder = local_var_req_builder.query(&[("fen", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = play {
        local_var_req_builder = local_var_req_builder.query(&[("play", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = speeds {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("speeds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("speeds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = modes {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("modes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("modes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = until {
        local_var_req_builder = local_var_req_builder.query(&[("until", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = moves {
        local_var_req_builder = local_var_req_builder.query(&[("moves", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recent_games {
        local_var_req_builder = local_var_req_builder.query(&[("recentGames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpeningExplorerPlayerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

