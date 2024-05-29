# \MessagingApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inbox_username**](MessagingApi.md#inbox_username) | **POST** /inbox/{username} | Send a private message



## inbox_username

> crate::models::Ok inbox_username(username, text)
Send a private message

Send a private message to another player. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**text** | **String** |  | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

