# GameJson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**rated** | **bool** |  | 
**variant** | [**crate::models::VariantKey**](VariantKey.md) |  | 
**speed** | [**crate::models::Speed**](Speed.md) |  | 
**perf** | **String** |  | 
**created_at** | **f32** |  | 
**last_move_at** | **f32** |  | 
**status** | [**crate::models::GameStatus**](GameStatus.md) |  | 
**players** | [**crate::models::GameJsonPlayers**](GameJson_players.md) |  | 
**initial_fen** | Option<**String**> |  | [optional]
**winner** | Option<**String**> |  | [optional]
**opening** | Option<[**crate::models::GameJsonOpening**](GameJson_opening.md)> |  | [optional]
**moves** | Option<**String**> |  | [optional]
**pgn** | Option<**String**> |  | [optional]
**days_per_turn** | Option<**f32**> |  | [optional]
**analysis** | Option<[**Vec<crate::models::GameJsonAnalysisInner>**](GameJson_analysis_inner.md)> |  | [optional]
**tournament** | Option<**String**> |  | [optional]
**swiss** | Option<**String**> |  | [optional]
**clock** | Option<[**crate::models::GameJsonClock**](GameJson_clock.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


