# GameFullEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**id** | **String** |  | 
**variant** | [**crate::models::Variant**](Variant.md) |  | 
**clock** | Option<[**crate::models::Clock**](Clock.md)> |  | 
**speed** | [**crate::models::Speed**](Speed.md) |  | 
**perf** | [**crate::models::GameFullEventPerf**](GameFullEvent_perf.md) |  | 
**rated** | **bool** |  | 
**created_at** | **f32** |  | 
**white** | [**crate::models::GameEventPlayer**](GameEventPlayer.md) |  | 
**black** | [**crate::models::GameEventPlayer**](GameEventPlayer.md) |  | 
**initial_fen** | **String** |  | [default to startpos]
**state** | [**crate::models::GameStateEvent**](GameStateEvent.md) |  | 
**tournament_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


