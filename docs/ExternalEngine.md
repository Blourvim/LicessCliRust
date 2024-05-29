# ExternalEngine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique engine registration ID. | 
**name** | **String** | Display name of the engine. | 
**client_secret** | **String** | A secret token that can be used to [*request* analysis](#tag/External-engine/operation/apiExternalEngineAnalyse) from this external engine.  | 
**user_id** | **String** | The user this engine has been registered for. | 
**max_threads** | **i32** | Maximum number of available threads. | 
**max_hash** | **i32** | Maximum available hash table size, in MiB. | 
**default_depth** | **i32** | Estimated depth of normal search. | 
**variants** | [**Vec<crate::models::UciVariant>**](UciVariant.md) | List of supported chess variants. | 
**provider_data** | Option<**String**> | Arbitrary data that the engine provider can use for identification or bookkeeping.  Users can read this information, but updating it requires knowing or changing the `providerSecret`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


