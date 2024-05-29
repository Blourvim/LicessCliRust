# GameStateEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**moves** | **String** | Current moves in UCI format | 
**wtime** | **i32** | Integer of milliseconds White has left on the clock | 
**btime** | **i32** | Integer of milliseconds Black has left on the clock | 
**winc** | **i32** | Integer of White Fisher increment. | 
**binc** | **i32** | Integer of Black Fisher increment. | 
**status** | [**crate::models::GameStatus**](GameStatus.md) |  | 
**winner** | Option<**String**> | Color of the winner, if any | [optional]
**wdraw** | Option<**bool**> | true if white is offering draw, else omitted | [optional]
**bdraw** | Option<**bool**> | true if black is offering draw, else omitted | [optional]
**wtakeback** | Option<**bool**> | true if white is proposing takeback, else omitted | [optional]
**btakeback** | Option<**bool**> | true if black is proposing takeback, else omitted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


