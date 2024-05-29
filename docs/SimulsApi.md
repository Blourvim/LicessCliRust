# \SimulsApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_simul**](SimulsApi.md#api_simul) | **GET** /api/simul | Get current simuls



## api_simul

> crate::models::ApiSimul200Response api_simul()
Get current simuls

Get recently created, started, finished, simuls.  Created and finished simul lists are not exhaustives, only those with strong enough host will be listed, the same filter is used to display simuls on https://lichess.org/simul.  When [authenticated with OAuth2](#section/Introduction/Authentication), the pending list will be populated with your created, but unstarted simuls. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiSimul200Response**](apiSimul_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

