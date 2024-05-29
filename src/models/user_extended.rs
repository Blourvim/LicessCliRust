/*
 * Lichess.org API reference
 *
 * # Introduction Welcome to the reference for the Lichess API! Lichess is free/libre, open-source chess server powered by volunteers and donations. - Get help in the [Lichess Discord channel](https://discord.gg/lichess) - API demo app with OAuth2 login and gameplay: [source](https://github.com/lichess-org/api-demo) / [demo](https://lichess-org.github.io/api-demo/) - API UI app with OAuth2 login and endpoint forms: [source](https://github.com/lichess-org/api-ui) / [website](https://lichess.org/api/ui) - [Contribute to this documentation on Github](https://github.com/lichess-org/api) - Check out [Lichess widgets to embed in your website](https://lichess.org/developers) - [Download all Lichess rated games](https://database.lichess.org/) - [Download all Lichess puzzles with themes, ratings and votes](https://database.lichess.org/#puzzles) - [Download all evaluated positions](https://database.lichess.org/#evals)  ## Endpoint All requests go to `https://lichess.org` (unless otherwise specified).  ## Clients - [Python general API](https://github.com/lichess-org/berserk) - [MicroPython general API](https://github.com/mkomon/uberserk) - [Python general API - async](https://pypi.org/project/async-lichess-sdk) - [Python Lichess Bot](https://github.com/lichess-bot-devs/lichess-bot) - [Python Board API for Certabo](https://github.com/haklein/certabo-lichess) - [Java general API](https://github.com/tors42/chariot) - [JavaScript & TypeScript general API](https://github.com/devjiwonchoi/equine)  ## Rate limiting All requests are rate limited using various strategies, to ensure the API remains responsive for everyone. Only make one request at a time. If you receive an HTTP response with a [429 status](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#429), please wait a full minute before resuming API usage.  ## Streaming with ND-JSON Some API endpoints stream their responses as [Newline Delimited JSON a.k.a. **nd-json**](http://ndjson.org/), with one JSON object per line.  Here's a [JavaScript utility function](https://gist.github.com/ornicar/a097406810939cf7be1df8ea30e94f3e) to help reading NDJSON streamed responses.  ## Authentication ### Which authentication method is right for me? [Read about the Lichess API authentication methods and code examples](https://github.com/lichess-org/api/blob/master/example/README.md)  ### Personal Access Token Personal API access tokens allow you to quickly interact with Lichess API without going through an OAuth flow. - [Generate a personal access token](https://lichess.org/account/oauth/token) - `curl https://lichess.org/api/account -H \"Authorization: Bearer {token}\"` - [NodeJS example](https://github.com/lichess-org/api/tree/master/example/oauth-personal-token)  ### Authorization Code Flow with PKCE The authorization code flow with PKCE allows your users to **login with Lichess**. Lichess supports unregistered and public clients (no client authentication, choose any unique client id). The only accepted code challenge method is `S256`. Access tokens are long-lived (expect one year), unless they are revoked. Refresh tokens are not supported.  See the [documentation for the OAuth endpoints](#tag/OAuth) or the [PKCE RFC](https://datatracker.ietf.org/doc/html/rfc7636#section-4) for a precise protocol description.  - [Demo app](https://lichess-org.github.io/api-demo/) - [Minimal client-side example](https://github.com/lichess-org/api/tree/master/example/oauth-app) - [Flask/Python example](https://github.com/lakinwecker/lichess-oauth-flask) - [Java example](https://github.com/tors42/lichess-oauth-pkce-app) - [NodeJS Passport strategy to login with Lichess OAuth2](https://www.npmjs.com/package/passport-lichess)  #### Real life examples - [PyChess](https://github.com/gbtami/pychess-variants) ([source code](https://github.com/gbtami/pychess-variants)) - [Lichess4545](https://www.lichess4545.com/) ([source code](https://github.com/cyanfish/heltour)) - [English Chess Federation](https://ecf.octoknight.com/) - [Rotherham Online Chess](https://rotherhamonlinechess.azurewebsites.net/tournaments)  ### Token format Access tokens and authorization codes match `^[A-Za-z0-9_]+$`. The length of tokens can be increased without notice. Make sure your application can handle at least 512 characters. By convention tokens have a recognizable prefix, but do not rely on this. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: contact@lichess.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserExtended {
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<serde_json::Value>>,
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<serde_json::Value>>,
    #[serde(rename = "perfs", skip_serializing_if = "Option::is_none")]
    pub perfs: Option<Box<crate::models::Perfs>>,
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<serde_json::Value>>,
    #[serde(rename = "disabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Option<serde_json::Value>>,
    #[serde(rename = "tosViolation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tos_violation: Option<Option<serde_json::Value>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::Profile>>,
    #[serde(rename = "seenAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<Option<serde_json::Value>>,
    #[serde(rename = "patron", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub patron: Option<Option<serde_json::Value>>,
    #[serde(rename = "verified", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verified: Option<Option<serde_json::Value>>,
    #[serde(rename = "playTime", skip_serializing_if = "Option::is_none")]
    pub play_time: Option<Box<crate::models::PlayTime>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<crate::models::Title>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "playing", skip_serializing_if = "Option::is_none")]
    pub playing: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<Box<crate::models::Count>>,
    #[serde(rename = "streaming", skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
    #[serde(rename = "streamer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub streamer: Option<Option<serde_json::Value>>,
    #[serde(rename = "followable", skip_serializing_if = "Option::is_none")]
    pub followable: Option<bool>,
    #[serde(rename = "following", skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(rename = "blocking", skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    #[serde(rename = "followsYou", skip_serializing_if = "Option::is_none")]
    pub follows_you: Option<bool>,
}

impl UserExtended {
    pub fn new() -> UserExtended {
        UserExtended {
            id: None,
            username: None,
            perfs: None,
            created_at: None,
            disabled: None,
            tos_violation: None,
            profile: None,
            seen_at: None,
            patron: None,
            verified: None,
            play_time: None,
            title: None,
            url: None,
            playing: None,
            count: None,
            streaming: None,
            streamer: None,
            followable: None,
            following: None,
            blocking: None,
            follows_you: None,
        }
    }
}


