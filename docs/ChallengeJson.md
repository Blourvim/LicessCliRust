# ChallengeJson

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**url** | **String** |  | 
**status** | **String** |  | 
**challenger** | [**crate::models::ChallengeUser**](ChallengeUser.md) |  | 
**dest_user** | Option<[**crate::models::ChallengeUser**](ChallengeUser.md)> |  | 
**variant** | [**crate::models::Variant**](Variant.md) |  | 
**rated** | **bool** |  | 
**speed** | [**crate::models::Speed**](Speed.md) |  | 
**time_control** | Option<[**crate::models::OneOfLessThanGreaterThan**](oneOf<>.md)> |  | 
**color** | **String** |  | 
**perf** | [**crate::models::ChallengeJsonPerf**](ChallengeJson_perf.md) |  | 
**direction** | Option<**String**> |  | [optional]
**initial_fen** | Option<**String**> |  | [optional]
**decline_reason** | Option<**String**> | Human readable, possibly translated reason why the challenge was declined. | [optional]
**decline_reason_key** | Option<**String**> | Untranslated, computer-matchable reason why the challenge was declined. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


