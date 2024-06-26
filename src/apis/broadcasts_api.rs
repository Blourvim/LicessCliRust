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

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`broadcast_all_rounds_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastAllRoundsPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastIndexError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_my_rounds_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastMyRoundsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_push`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastPushError {
    Status400(crate::models::BroadcastPush400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_round_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastRoundCreateError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_round_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastRoundGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_round_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastRoundPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_round_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastRoundUpdateError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_stream_round_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastStreamRoundPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_tour_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastTourCreateError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_tour_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastTourGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_tour_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastTourUpdateError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Download all games of all rounds of a broadcast in PGN format.  If a `study:read` [OAuth token](#tag/OAuth) is provided, the private rounds where the user is a contributor will be available.  You may want to [download only the games of a single round](#operation/broadcastRoundPgn) instead. 
pub async fn broadcast_all_rounds_pgn(configuration: &configuration::Configuration, broadcast_tournament_id: &str) -> Result<serde_json::Value, Error<BroadcastAllRoundsPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/broadcast/{broadcastTournamentId}.pgn", local_var_configuration.base_path, broadcastTournamentId=crate::apis::urlencode(broadcast_tournament_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastAllRoundsPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all incoming, ongoing, and finished official broadcasts. The broadcasts are sorted by start date, most recent first.  Broadcasts are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 
pub async fn broadcast_index(configuration: &configuration::Configuration, nb: Option<i32>) -> Result<Vec<serde_json::Value>, Error<BroadcastIndexError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/broadcast", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = nb {
        local_var_req_builder = local_var_req_builder.query(&[("nb", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BroadcastIndexError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stream all broadcast rounds you are a member of. Also includes broadcasts rounds you did not create, but were invited to. Also includes broadcasts rounds where you're a non-writing member. See the `writeable` flag in the response. Rounds are ordered by rank, which is roughly chronological, most recent first, slightly pondered with popularity. 
pub async fn broadcast_my_rounds_get(configuration: &configuration::Configuration, nb: Option<i32>) -> Result<serde_json::Value, Error<BroadcastMyRoundsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/broadcast/my-rounds", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = nb {
        local_var_req_builder = local_var_req_builder.query(&[("nb", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastMyRoundsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update your broadcast with new PGN. Only for broadcast without a source URL. 
pub async fn broadcast_push(configuration: &configuration::Configuration, broadcast_round_id: &str, body: &str) -> Result<crate::models::BroadcastPush200Response, Error<BroadcastPushError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/round/{broadcastRoundId}/push", local_var_configuration.base_path, broadcastRoundId=crate::apis::urlencode(broadcast_round_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastPushError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new broadcast round to relay external games. This endpoint accepts the same form data as the web form. 
pub async fn broadcast_round_create(configuration: &configuration::Configuration, broadcast_tournament_id: &str, name: &str, sync_url: Option<&str>, sync_url_round: Option<&str>, starts_at: Option<i32>, delay: Option<i32>, period: Option<i32>, finished: Option<bool>) -> Result<serde_json::Value, Error<BroadcastRoundCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/{broadcastTournamentId}/new", local_var_configuration.base_path, broadcastTournamentId=crate::apis::urlencode(broadcast_tournament_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("name", name.to_string());
    if let Some(local_var_param_value) = sync_url {
        local_var_form_params.insert("syncUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sync_url_round {
        local_var_form_params.insert("syncUrlRound", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = starts_at {
        local_var_form_params.insert("startsAt", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = delay {
        local_var_form_params.insert("delay", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = finished {
        local_var_form_params.insert("finished", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastRoundCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a broadcast round. 
pub async fn broadcast_round_get(configuration: &configuration::Configuration, broadcast_tournament_slug: &str, broadcast_round_slug: &str, broadcast_round_id: &str) -> Result<serde_json::Value, Error<BroadcastRoundGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/broadcast/{broadcastTournamentSlug}/{broadcastRoundSlug}/{broadcastRoundId}", local_var_configuration.base_path, broadcastTournamentSlug=crate::apis::urlencode(broadcast_tournament_slug), broadcastRoundSlug=crate::apis::urlencode(broadcast_round_slug), broadcastRoundId=crate::apis::urlencode(broadcast_round_id));
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
        let local_var_entity: Option<BroadcastRoundGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Download all games of a single round of a broadcast tournament in PGN format.  You *could* poll this endpoint to get updates about a tournament, but it would be slow, and very inneficient.  Instead, consider [streaming the tournament](#operation/broadcastStreamRoundPgn) to get a new PGN every time a game is updated, in real-time. 
pub async fn broadcast_round_pgn(configuration: &configuration::Configuration, broadcast_round_id: &str) -> Result<serde_json::Value, Error<BroadcastRoundPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/broadcast/round/{broadcastRoundId}.pgn", local_var_configuration.base_path, broadcastRoundId=crate::apis::urlencode(broadcast_round_id));
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
        let local_var_entity: Option<BroadcastRoundPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update information about a broadcast round that you created. This endpoint accepts the same form data as the web form. All fields must be populated with data. Missing fields will override the broadcast with empty data. For instance, if you omit `startDate`, then any pre-existing start date will be removed. 
pub async fn broadcast_round_update(configuration: &configuration::Configuration, broadcast_round_id: &str, name: &str, sync_url: Option<&str>, sync_url_round: Option<&str>, starts_at: Option<i32>, delay: Option<i32>, period: Option<i32>, finished: Option<bool>) -> Result<crate::models::Ok, Error<BroadcastRoundUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/round/{broadcastRoundId}/edit", local_var_configuration.base_path, broadcastRoundId=crate::apis::urlencode(broadcast_round_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("name", name.to_string());
    if let Some(local_var_param_value) = sync_url {
        local_var_form_params.insert("syncUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sync_url_round {
        local_var_form_params.insert("syncUrlRound", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = starts_at {
        local_var_form_params.insert("startsAt", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = delay {
        local_var_form_params.insert("delay", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = finished {
        local_var_form_params.insert("finished", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastRoundUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This streaming endpoint first sends all games of a broadcast tournament in PGN format.  Then, it waits for new moves to be played. As soon as it happens, the entire PGN of the game is sent to the stream.  The stream will also send PGNs when games are added to the tournament.  This is the best way to get updates about an ongoing tournament. Streaming means no polling, and no pollings means no latency, and minimum impact on the server. 
pub async fn broadcast_stream_round_pgn(configuration: &configuration::Configuration, broadcast_round_id: &str) -> Result<serde_json::Value, Error<BroadcastStreamRoundPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/stream/broadcast/round/{broadcastRoundId}.pgn", local_var_configuration.base_path, broadcastRoundId=crate::apis::urlencode(broadcast_round_id));
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
        let local_var_entity: Option<BroadcastStreamRoundPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new broadcast tournament to relay external games. This endpoint accepts the same form data as the [web form](https://lichess.org/broadcast/new). 
pub async fn broadcast_tour_create(configuration: &configuration::Configuration, name: &str, description: &str, auto_leaderboard: bool, markdown: Option<&str>, tier: Option<i32>, players: Option<serde_json::Value>) -> Result<serde_json::Value, Error<BroadcastTourCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/new", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("name", name.to_string());
    local_var_form_params.insert("description", description.to_string());
    local_var_form_params.insert("autoLeaderboard", auto_leaderboard.to_string());
    if let Some(local_var_param_value) = markdown {
        local_var_form_params.insert("markdown", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tier {
        local_var_form_params.insert("tier", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = players {
        local_var_form_params.insert("players", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastTourCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a broadcast tournament. 
pub async fn broadcast_tour_get(configuration: &configuration::Configuration, slug: &str, broadcast_tournament_id: &str) -> Result<serde_json::Value, Error<BroadcastTourGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/{slug}/{broadcastTournamentId}", local_var_configuration.base_path, slug=crate::apis::urlencode(slug), broadcastTournamentId=crate::apis::urlencode(broadcast_tournament_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastTourGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update information about a broadcast tournament that you created. This endpoint accepts the same form data as the web form. All fields must be populated with data. Missing fields will override the broadcast with empty data. 
pub async fn broadcast_tour_update(configuration: &configuration::Configuration, broadcast_tournament_id: &str, name: &str, description: &str, auto_leaderboard: bool, markdown: Option<&str>, tier: Option<i32>, players: Option<serde_json::Value>) -> Result<crate::models::Ok, Error<BroadcastTourUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/broadcast/{broadcastTournamentId}/edit", local_var_configuration.base_path, broadcastTournamentId=crate::apis::urlencode(broadcast_tournament_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("name", name.to_string());
    local_var_form_params.insert("description", description.to_string());
    local_var_form_params.insert("autoLeaderboard", auto_leaderboard.to_string());
    if let Some(local_var_param_value) = markdown {
        local_var_form_params.insert("markdown", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tier {
        local_var_form_params.insert("tier", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = players {
        local_var_form_params.insert("players", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastTourUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

