# GameJsonAnalysisInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**eval** | Option<**f32**> | Evaluation in centipawns | [optional]
**mate** | Option<**f32**> | Number of moves until forced mate | [optional]
**best** | Option<**String**> | Best move in UCI notation (only if played move was inaccurate) | [optional]
**variation** | Option<**String**> | Best variation in SAN notation (only if played move was inaccurate) | [optional]
**judgment** | Option<[**crate::models::GameJsonAnalysisInnerJudgment**](GameJson_analysis_inner_judgment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


