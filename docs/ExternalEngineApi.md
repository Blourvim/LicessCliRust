# \ExternalEngineApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_external_engine_acquire**](ExternalEngineApi.md#api_external_engine_acquire) | **POST** /api/external-engine/work | Acquire analysis request
[**api_external_engine_analyse**](ExternalEngineApi.md#api_external_engine_analyse) | **POST** /api/external-engine/{id}/analyse | Analyse with external engine
[**api_external_engine_create**](ExternalEngineApi.md#api_external_engine_create) | **POST** /api/external-engine | Create external engine
[**api_external_engine_delete**](ExternalEngineApi.md#api_external_engine_delete) | **DELETE** /api/external-engine/{id} | Delete external engine
[**api_external_engine_get**](ExternalEngineApi.md#api_external_engine_get) | **GET** /api/external-engine/{id} | Get external engine
[**api_external_engine_list**](ExternalEngineApi.md#api_external_engine_list) | **GET** /api/external-engine | List external engines
[**api_external_engine_put**](ExternalEngineApi.md#api_external_engine_put) | **PUT** /api/external-engine/{id} | Update external engine
[**api_external_engine_submit**](ExternalEngineApi.md#api_external_engine_submit) | **POST** /api/external-engine/work/{id} | Answer analysis request



## api_external_engine_acquire

> crate::models::ApiExternalEngineAcquire200Response api_external_engine_acquire(api_external_engine_acquire_request)
Acquire analysis request

**Endpoint: `https://engine.lichess.ovh/api/external-engine/work`**  Wait for an analysis requests to any of the external engines that have been registered with the given `secret`.  Uses long polling.  After acquiring a request, the provider should immediately [start streaming the results](#tag/External-engine/operation/apiExternalEngineSubmit). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_external_engine_acquire_request** | [**ApiExternalEngineAcquireRequest**](ApiExternalEngineAcquireRequest.md) | Provider credentials. | [required] |

### Return type

[**crate::models::ApiExternalEngineAcquire200Response**](apiExternalEngineAcquire_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_analyse

> crate::models::ApiExternalEngineAnalyse200Response api_external_engine_analyse(id, api_external_engine_analyse_request)
Analyse with external engine

**Endpoint: `https://engine.lichess.ovh/api/external-engine/{id}/analyse`**  Request analysis from an external engine.  Response content is streamed as [newline delimited JSON](#section/Introduction/Streaming-with-ND-JSON). The properties are based on the [UCI specification](https://backscattering.de/chess/uci/#engine). Analysis stops when the client goes away, the requested limit is reached, or the provider goes away. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The external engine id. | [required] |
**api_external_engine_analyse_request** | [**ApiExternalEngineAnalyseRequest**](ApiExternalEngineAnalyseRequest.md) | Engine credentials and analysis request. | [required] |

### Return type

[**crate::models::ApiExternalEngineAnalyse200Response**](apiExternalEngineAnalyse_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_create

> crate::models::ExternalEngine api_external_engine_create(external_engine_registration)
Create external engine

Registers a new external engine for the user. It can then be selected and used on the analysis board.  After registering, the provider should start waiting for analyis requests. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_engine_registration** | [**ExternalEngineRegistration**](ExternalEngineRegistration.md) | A new external engine registration. | [required] |

### Return type

[**crate::models::ExternalEngine**](ExternalEngine.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_delete

> crate::models::Ok api_external_engine_delete(id)
Delete external engine

Unregisters an external engine. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The external engine id. | [required] |

### Return type

[**crate::models::Ok**](Ok.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_get

> crate::models::ExternalEngine api_external_engine_get(id)
Get external engine

Get properties and credentials of an external engine. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The external engine id. | [required] |

### Return type

[**crate::models::ExternalEngine**](ExternalEngine.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_list

> Vec<crate::models::ExternalEngine> api_external_engine_list()
List external engines

Lists all external engines that have been registered for the user, and the credentials required to use them. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ExternalEngine>**](ExternalEngine.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_put

> crate::models::ExternalEngine api_external_engine_put(id, external_engine_registration)
Update external engine

Updates the properties of an external engine. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The external engine id. | [required] |
**external_engine_registration** | [**ExternalEngineRegistration**](ExternalEngineRegistration.md) | A modified engine registration. | [required] |

### Return type

[**crate::models::ExternalEngine**](ExternalEngine.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_external_engine_submit

> api_external_engine_submit(id, body)
Answer analysis request

**Endpoint: `https://engine.lichess.ovh/api/external-engine/work/{id}`**  Submit a stream of analysis as [UCI output](https://backscattering.de/chess/uci/#engine-info).  * The engine should always be in `UCI_Chess960` mode. * `UCI_AnalyseMode` enabled if available. * It produces `info` with at least:   - `depth`   - `multipv` (between 1 and 5)   - `score`   - `nodes`   - `time`   - `pv`  The server may close the connection at any time, indicating that the requester has gone away and analysis should be stopped. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **String** | Analysis results | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

