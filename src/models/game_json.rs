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
pub struct GameJson {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "rated")]
    pub rated: bool,
    #[serde(rename = "variant")]
    pub variant: crate::models::VariantKey,
    #[serde(rename = "speed")]
    pub speed: crate::models::Speed,
    #[serde(rename = "perf")]
    pub perf: String,
    #[serde(rename = "createdAt")]
    pub created_at: f32,
    #[serde(rename = "lastMoveAt")]
    pub last_move_at: f32,
    #[serde(rename = "status")]
    pub status: crate::models::GameStatus,
    #[serde(rename = "players")]
    pub players: Box<crate::models::GameJsonPlayers>,
    #[serde(rename = "initialFen", skip_serializing_if = "Option::is_none")]
    pub initial_fen: Option<String>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<Winner>,
    #[serde(rename = "opening", skip_serializing_if = "Option::is_none")]
    pub opening: Option<Box<crate::models::GameJsonOpening>>,
    #[serde(rename = "moves", skip_serializing_if = "Option::is_none")]
    pub moves: Option<String>,
    #[serde(rename = "pgn", skip_serializing_if = "Option::is_none")]
    pub pgn: Option<String>,
    #[serde(rename = "daysPerTurn", skip_serializing_if = "Option::is_none")]
    pub days_per_turn: Option<f32>,
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Vec<crate::models::GameJsonAnalysisInner>>,
    #[serde(rename = "tournament", skip_serializing_if = "Option::is_none")]
    pub tournament: Option<String>,
    #[serde(rename = "swiss", skip_serializing_if = "Option::is_none")]
    pub swiss: Option<String>,
    #[serde(rename = "clock", skip_serializing_if = "Option::is_none")]
    pub clock: Option<Box<crate::models::GameJsonClock>>,
}

impl GameJson {
    pub fn new(id: String, rated: bool, variant: crate::models::VariantKey, speed: crate::models::Speed, perf: String, created_at: f32, last_move_at: f32, status: crate::models::GameStatus, players: crate::models::GameJsonPlayers) -> GameJson {
        GameJson {
            id,
            rated,
            variant,
            speed,
            perf,
            created_at,
            last_move_at,
            status,
            players: Box::new(players),
            initial_fen: None,
            winner: None,
            opening: None,
            moves: None,
            pgn: None,
            days_per_turn: None,
            analysis: None,
            tournament: None,
            swiss: None,
            clock: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Winner {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

impl Default for Winner {
    fn default() -> Winner {
        Self::White
    }
}
