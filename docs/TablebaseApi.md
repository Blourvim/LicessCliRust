# \TablebaseApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**antichess_atomic**](TablebaseApi.md#antichess_atomic) | **GET** /antichess | Tablebase lookup for Antichess
[**tablebase_atomic**](TablebaseApi.md#tablebase_atomic) | **GET** /atomic | Tablebase lookup for Atomic chess
[**tablebase_standard**](TablebaseApi.md#tablebase_standard) | **GET** /standard | Tablebase lookup



## antichess_atomic

> String antichess_atomic()
Tablebase lookup for Antichess

**Endpoint: <https://tablebase.lichess.ovh>** 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablebase_atomic

> String tablebase_atomic()
Tablebase lookup for Atomic chess

**Endpoint: <https://tablebase.lichess.ovh>** 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablebase_standard

> crate::models::TablebaseJson tablebase_standard(fen)
Tablebase lookup

**Endpoint: <https://tablebase.lichess.ovh>**  Example: `curl http://tablebase.lichess.ovh/standard?fen=4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fen** | **String** | FEN of the position. Underscores allowed. | [required] |

### Return type

[**crate::models::TablebaseJson**](TablebaseJson.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

