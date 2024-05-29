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


/// struct for typed errors of method [`api_study_import_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiStudyImportPgnError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`study_all_chapters_head`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StudyAllChaptersHeadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`study_all_chapters_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StudyAllChaptersPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`study_chapter_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StudyChapterPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`study_export_all_pgn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StudyExportAllPgnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`study_list_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StudyListMetadataError {
    UnknownValue(serde_json::Value),
}


/// Imports arbitrary PGN into an existing [study](https://lichess.org/study). Creates a new chapter in the study.  If the PGN contains multiple games (separated by 2 or more newlines) then multiple chapters will be created within the study.  Note that a study can contain at most 64 chapters. 
pub async fn api_study_import_pgn(configuration: &configuration::Configuration, study_id: &str, name: &str, pgn: &str, orientation: Option<&str>, variant: Option<crate::models::VariantKey>) -> Result<serde_json::Value, Error<ApiStudyImportPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/study/{studyId}/import-pgn", local_var_configuration.base_path, studyId=crate::apis::urlencode(study_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("name", name.to_string());
    local_var_form_params.insert("pgn", pgn.to_string());
    if let Some(local_var_param_value) = orientation {
        local_var_form_params.insert("orientation", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = variant {
        local_var_form_params.insert("variant", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiStudyImportPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Only get the study headers, including `Last-Modified`. 
pub async fn study_all_chapters_head(configuration: &configuration::Configuration, study_id: &str) -> Result<(), Error<StudyAllChaptersHeadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/study/{studyId}.pgn", local_var_configuration.base_path, studyId=crate::apis::urlencode(study_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::HEAD, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StudyAllChaptersHeadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Download all chapters of a study in PGN format. 
pub async fn study_all_chapters_pgn(configuration: &configuration::Configuration, study_id: &str, clocks: Option<bool>, comments: Option<bool>, variations: Option<bool>, source: Option<bool>, orientation: Option<bool>) -> Result<serde_json::Value, Error<StudyAllChaptersPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/study/{studyId}.pgn", local_var_configuration.base_path, studyId=crate::apis::urlencode(study_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = clocks {
        local_var_req_builder = local_var_req_builder.query(&[("clocks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = comments {
        local_var_req_builder = local_var_req_builder.query(&[("comments", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = variations {
        local_var_req_builder = local_var_req_builder.query(&[("variations", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = source {
        local_var_req_builder = local_var_req_builder.query(&[("source", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orientation {
        local_var_req_builder = local_var_req_builder.query(&[("orientation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<StudyAllChaptersPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Download one study chapter in PGN format. 
pub async fn study_chapter_pgn(configuration: &configuration::Configuration, study_id: &str, chapter_id: &str, clocks: Option<bool>, comments: Option<bool>, variations: Option<bool>, source: Option<bool>, orientation: Option<bool>) -> Result<serde_json::Value, Error<StudyChapterPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/study/{studyId}/{chapterId}.pgn", local_var_configuration.base_path, studyId=crate::apis::urlencode(study_id), chapterId=crate::apis::urlencode(chapter_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = clocks {
        local_var_req_builder = local_var_req_builder.query(&[("clocks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = comments {
        local_var_req_builder = local_var_req_builder.query(&[("comments", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = variations {
        local_var_req_builder = local_var_req_builder.query(&[("variations", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = source {
        local_var_req_builder = local_var_req_builder.query(&[("source", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orientation {
        local_var_req_builder = local_var_req_builder.query(&[("orientation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<StudyChapterPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Download all chapters of all studies of a user in PGN format.  If authenticated, then all public, unlisted, and private studies are included.  If not, only public (non-unlisted) studies are included. 
pub async fn study_export_all_pgn(configuration: &configuration::Configuration, username: &str, clocks: Option<bool>, comments: Option<bool>, variations: Option<bool>, source: Option<bool>, orientation: Option<bool>) -> Result<serde_json::Value, Error<StudyExportAllPgnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/study/by/{username}/export.pgn", local_var_configuration.base_path, username=crate::apis::urlencode(username));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = clocks {
        local_var_req_builder = local_var_req_builder.query(&[("clocks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = comments {
        local_var_req_builder = local_var_req_builder.query(&[("comments", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = variations {
        local_var_req_builder = local_var_req_builder.query(&[("variations", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = source {
        local_var_req_builder = local_var_req_builder.query(&[("source", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orientation {
        local_var_req_builder = local_var_req_builder.query(&[("orientation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<StudyExportAllPgnError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get metadata (name and dates) of all studies of a user.  If authenticated, then all public, unlisted, and private studies are included.  If not, only public (non-unlisted) studies are included.  Studies are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 
pub async fn study_list_metadata(configuration: &configuration::Configuration, username: &str) -> Result<Vec<serde_json::Value>, Error<StudyListMetadataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/study/by/{username}", local_var_configuration.base_path, username=crate::apis::urlencode(username));
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
        let local_var_entity: Option<StudyListMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
