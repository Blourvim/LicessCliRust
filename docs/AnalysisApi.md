# \AnalysisApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_cloud_eval**](AnalysisApi.md#api_cloud_eval) | **GET** /api/cloud-eval | Get cloud evaluation of a position.



## api_cloud_eval

> api_cloud_eval(fen, multi_pv, variant)
Get cloud evaluation of a position.

Get the cached evaluation of a position, if available.  Opening positions have more chances of being available. There are about 15 million positions in the database.  Up to 5 variations may be available. Variants are supported.  Use this endpoint to fetch a few positions here and there. If you want to download a lot of positions, [get the full list](https://database.lichess.org/#evals) from our exported database. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fen** | **String** | FEN of the position | [required] |
**multi_pv** | Option<**f32**> | Number of variations |  |[default to 1]
**variant** | Option<[**VariantKey**](.md)> | Variant |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

