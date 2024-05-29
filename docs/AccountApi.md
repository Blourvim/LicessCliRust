# \AccountApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account**](AccountApi.md#account) | **GET** /api/account/preferences | Get my preferences
[**account_email**](AccountApi.md#account_email) | **GET** /api/account/email | Get my email address
[**account_kid**](AccountApi.md#account_kid) | **GET** /api/account/kid | Get my kid mode status
[**account_kid_post**](AccountApi.md#account_kid_post) | **POST** /api/account/kid | Set my kid mode status
[**account_me**](AccountApi.md#account_me) | **GET** /api/account | Get my profile
[**timeline**](AccountApi.md#timeline) | **GET** /api/timeline | Get my timeline



## account

> crate::models::Account200Response account()
Get my preferences

Read the preferences of the logged in user.  - <https://lichess.org/account/preferences/game-display> - <https://github.com/ornicar/lila/blob/master/modules/pref/src/main/Pref.scala> 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Account200Response**](account_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_email

> crate::models::AccountEmail200Response account_email()
Get my email address

Read the email address of the logged in user. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountEmail200Response**](accountEmail_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_kid

> crate::models::AccountKid200Response account_kid()
Get my kid mode status

Read the kid mode status of the logged in user.  - <https://lichess.org/account/kid> 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountKid200Response**](accountKid_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_kid_post

> crate::models::Ok account_kid_post(v)
Set my kid mode status

Set the kid mode status of the logged in user.  - <https://lichess.org/account/kid> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v** | **bool** | Kid mode status | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_me

> crate::models::UserExtended account_me()
Get my profile

Public information about the logged in user. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserExtended**](UserExtended.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timeline

> crate::models::Timeline timeline(since, nb)
Get my timeline

Get the timeline events of the logged in user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i32**> | Show events since this timestamp. |  |
**nb** | Option<**i32**> | Max number of events to fetch. |  |[default to 15]

### Return type

[**crate::models::Timeline**](Timeline.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

