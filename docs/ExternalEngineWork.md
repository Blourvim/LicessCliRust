# ExternalEngineWork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **String** | Arbitary string that identifies the analysis session. Providers may wish to clear the hash table between sessions.  | 
**threads** | **i32** | Number of threads to use for analysis. | 
**hash** | **i32** | Hash table size to use for analysis, in MiB. | 
**infinite** | Option<**bool**> | Request an infinite search (rather than roughly aiming for `defaultDepth`).  | [optional]
**multi_pv** | **i32** | Requested number of principal variations. | 
**variant** | [**crate::models::UciVariant**](UciVariant.md) |  | 
**initial_fen** | **String** | Initial position of the game. | 
**moves** | **Vec<String>** | List of moves played from the initial position, in UCI notation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


