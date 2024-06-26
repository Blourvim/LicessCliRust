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
pub struct ArenaTournament {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(rename = "minutes", skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i32>,
    #[serde(rename = "clock", skip_serializing_if = "Option::is_none")]
    pub clock: Option<Box<crate::models::Clock>>,
    #[serde(rename = "rated", skip_serializing_if = "Option::is_none")]
    pub rated: Option<bool>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "nbPlayers", skip_serializing_if = "Option::is_none")]
    pub nb_players: Option<i32>,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<Box<crate::models::Variant>>,
    #[serde(rename = "startsAt", skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<i32>,
    #[serde(rename = "finishesAt", skip_serializing_if = "Option::is_none")]
    pub finishes_at: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ArenaStatus>,
    #[serde(rename = "perf", skip_serializing_if = "Option::is_none")]
    pub perf: Option<Box<crate::models::ArenaPerf>>,
    #[serde(rename = "secondsToStart", skip_serializing_if = "Option::is_none")]
    pub seconds_to_start: Option<i32>,
    #[serde(rename = "hasMaxRating", skip_serializing_if = "Option::is_none")]
    pub has_max_rating: Option<bool>,
    #[serde(rename = "maxRating", skip_serializing_if = "Option::is_none")]
    pub max_rating: Option<Box<crate::models::ArenaRatingObj>>,
    #[serde(rename = "minRating", skip_serializing_if = "Option::is_none")]
    pub min_rating: Option<Box<crate::models::ArenaRatingObj>>,
    #[serde(rename = "minRatedGames", skip_serializing_if = "Option::is_none")]
    pub min_rated_games: Option<Box<crate::models::ArenaTournamentMinRatedGames>>,
    #[serde(rename = "onlyTitled", skip_serializing_if = "Option::is_none")]
    pub only_titled: Option<bool>,
    #[serde(rename = "teamMember", skip_serializing_if = "Option::is_none")]
    pub team_member: Option<String>,
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(rename = "position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<Box<crate::models::ArenaPosition>>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::ArenaTournamentSchedule>>,
    #[serde(rename = "teamBattle", skip_serializing_if = "Option::is_none")]
    pub team_battle: Option<Box<crate::models::ArenaTournamentTeamBattle>>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<Box<crate::models::ArenaTournamentWinner>>,
}

impl ArenaTournament {
    pub fn new() -> ArenaTournament {
        ArenaTournament {
            id: None,
            created_by: None,
            system: None,
            minutes: None,
            clock: None,
            rated: None,
            full_name: None,
            nb_players: None,
            variant: None,
            starts_at: None,
            finishes_at: None,
            status: None,
            perf: None,
            seconds_to_start: None,
            has_max_rating: None,
            max_rating: None,
            min_rating: None,
            min_rated_games: None,
            only_titled: None,
            team_member: None,
            private: None,
            position: None,
            schedule: None,
            team_battle: None,
            winner: None,
        }
    }
}


