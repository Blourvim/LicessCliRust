# ExternalEngineRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Display name of the engine. | 
**max_threads** | **i32** | Maximum number of available threads. | 
**max_hash** | **i32** | Maximum available hash table size, in MiB. | 
**default_depth** | **i32** | Estimated depth of normal search. | 
**variants** | Option<[**Vec<crate::models::UciVariant>**](UciVariant.md)> | Optional list of supported chess variants. | [optional]
**provider_secret** | **String** | A random token that can be used to [wait for analysis requests](#tag/External-engine/operation/apiExternalEngineAcquire) and provide analysis.  The engine provider should securely generate a random string.  The token will not be readable again, even by the user.  The analysis provider can register multiple engines with the same token, even for different users, and wait for analysis requests from any of them. In this case, the request must not be made via CORS, so that the token is not revealed to any of the users.  | 
**provider_data** | Option<**String**> | Arbitrary data that the engine provider can use for identification or bookkeeping.  Users can read this information, but updating it requires knowing or changing the `providerSecret`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


