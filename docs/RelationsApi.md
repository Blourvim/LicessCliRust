# \RelationsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_user_following**](RelationsApi.md#api_user_following) | **GET** /api/rel/following | Get users followed by the logged in user
[**follow_user**](RelationsApi.md#follow_user) | **POST** /api/rel/follow/{username} | Follow a player
[**unfollow_user**](RelationsApi.md#unfollow_user) | **POST** /api/rel/unfollow/{username} | Unfollow a player



## api_user_following

> crate::models::UserExtended api_user_following()
Get users followed by the logged in user

Users are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserExtended**](UserExtended.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_user

> crate::models::Ok follow_user(username)
Follow a player

Follow a player, adding them to your list of Lichess friends. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_user

> crate::models::Ok unfollow_user(username)
Unfollow a player

Unfollow a player, removing them from your list of Lichess friends. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

