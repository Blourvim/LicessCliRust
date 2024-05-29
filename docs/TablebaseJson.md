# TablebaseJson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | Option<**String**> | `cursed-win` and `blessed-loss` means the 50-move rule prevents the decisive result.  `maybe-win` and `maybe-loss` means exact result is unknown due to [DTZ rounding](https://syzygy-tables.info/metrics#dtz), i.e., the win or loss could also be prevented by the 50-move rule if the user has deviated from the tablebase recommendation since the last pawn move or capture.  | [optional]
**dtz** | Option<**i32**> | [DTZ50'' with rounding](https://syzygy-tables.info/metrics#dtz) or null if unknown  | [optional]
**precise_dtz** | Option<**i32**> | DTZ50'' (only if guaranteed to be not rounded) or null if unknown  | [optional]
**dtm** | Option<**i32**> | Distance to mate (only for positions with not more than 5 pieces) | [optional]
**checkmate** | Option<**bool**> |  | [optional]
**stalemate** | Option<**bool**> |  | [optional]
**variant_win** | Option<**bool**> | Only in chess variants | [optional]
**variant_loss** | Option<**bool**> | Only in chess variants | [optional]
**insufficient_material** | Option<**bool**> |  | [optional]
**moves** | Option<[**Vec<crate::models::Move>**](Move.md)> | Information about legal moves, best first | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


