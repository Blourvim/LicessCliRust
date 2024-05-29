# BoardGameStream200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**serde_json::Value**](.md)> |  | 
**id** | Option<[**serde_json::Value**](.md)> |  | 
**variant** | [**crate::models::Variant**](Variant.md) |  | 
**clock** | [**crate::models::Clock**](Clock.md) |  | 
**speed** | [**crate::models::Speed**](Speed.md) |  | 
**perf** | [**crate::models::GameFullEventPerf**](GameFullEvent_perf.md) |  | 
**rated** | Option<[**serde_json::Value**](.md)> |  | 
**created_at** | Option<[**serde_json::Value**](.md)> |  | 
**white** | [**crate::models::GameEventPlayer**](GameEventPlayer.md) |  | 
**black** | [**crate::models::GameEventPlayer**](GameEventPlayer.md) |  | 
**initial_fen** | Option<[**serde_json::Value**](.md)> |  | [default to startpos]
**state** | [**crate::models::GameStateEvent**](GameStateEvent.md) |  | 
**tournament_id** | Option<[**serde_json::Value**](.md)> |  | [optional]
**moves** | Option<[**serde_json::Value**](.md)> | Current moves in UCI format | 
**wtime** | Option<[**serde_json::Value**](.md)> | Integer of milliseconds White has left on the clock | 
**btime** | Option<[**serde_json::Value**](.md)> | Integer of milliseconds Black has left on the clock | 
**winc** | Option<[**serde_json::Value**](.md)> | Integer of White Fisher increment. | 
**binc** | Option<[**serde_json::Value**](.md)> | Integer of Black Fisher increment. | 
**status** | [**crate::models::GameStatus**](GameStatus.md) |  | 
**winner** | Option<[**serde_json::Value**](.md)> | Color of the winner, if any | [optional]
**wdraw** | Option<[**serde_json::Value**](.md)> | true if white is offering draw, else omitted | [optional]
**bdraw** | Option<[**serde_json::Value**](.md)> | true if black is offering draw, else omitted | [optional]
**wtakeback** | Option<[**serde_json::Value**](.md)> | true if white is proposing takeback, else omitted | [optional]
**btakeback** | Option<[**serde_json::Value**](.md)> | true if black is proposing takeback, else omitted | [optional]
**room** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | 
**username** | Option<[**serde_json::Value**](.md)> |  | 
**text** | Option<[**serde_json::Value**](.md)> |  | 
**gone** | Option<[**serde_json::Value**](.md)> |  | 
**claim_win_in_seconds** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


