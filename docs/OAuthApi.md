# \OAuthApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_token**](OAuthApi.md#api_token) | **POST** /api/token | Obtain access token
[**api_token_delete**](OAuthApi.md#api_token_delete) | **DELETE** /api/token | Revoke access token
[**oauth**](OAuthApi.md#oauth) | **GET** /oauth | Request authorization code
[**token_test**](OAuthApi.md#token_test) | **POST** /api/token/test | Test multiple OAuth tokens



## api_token

> api_token(grant_type, code, code_verifier, redirect_uri, client_id)
Obtain access token

OAuth2 token endpoint. Exchanges an authorization code for an access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | Option<**String**> | Must be `authorization_code`. |  |
**code** | Option<**String**> | The authorization code that was sent in the `code` parameter to your `redirect_uri`. |  |
**code_verifier** | Option<**String**> | A `code_challenge` was used to request the authorization code. This must be the `code_verifier` it was derived from. |  |
**redirect_uri** | Option<**String**> | Must match the `redirect_uri` used to request the authorization code. |  |
**client_id** | Option<**String**> | Must match the `client_id` used to request the authorization code. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_token_delete

> api_token_delete()
Revoke access token

Revokes the access token sent as Bearer for this request.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth

> oauth(response_type, client_id, redirect_uri, code_challenge_method, code_challenge, scope, username, state)
Request authorization code

OAuth2 authorization endpoint.  Start the OAuth2 Authorization Code Flow with PKCE by securely generating two random strings unique to each authorization request: * `code_verifier` * `state`  Store these in session storage. Make sure not to reveal `code_verifier` to eavesdroppers. Do not show it in URLs, do not abuse `state` to store it, do not send it over insecure connections. However it is fine if the user themselves can extract `code_verifier`, which will always be possible for fully client-side apps.  Then send the user to this endpoint. They will be prompted to grant authorization and then be redirected back to the given `redirect_uri`.  If the authorization failed, the following query string parameters will be appended to the redirection: * `error`, in particular with value `access_denied` if the user    cancelled authorization * `error_description` to aid debugging * `state`, exactly as passed in the `state` parameter  If the authorization succeeded, the following query string parameters will be appended to the redirection: * `code`, containing a fresh short-lived authorization code * `state`, exactly as passed in the `state` parameter  Next, to defend against cross site request forgery, check that the returned `state` matches the `state` you originally generated.  Finally, continue by using the authorization code to [obtain an access token](#operation/apiToken). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_type** | **String** | Must be `code`. | [required] |
**client_id** | **String** | Arbitrary identifier that uniquely identifies your application. | [required] |
**redirect_uri** | **String** | The absolute URL that the user should be redirected to with the authorization result. | [required] |
**code_challenge_method** | **String** | Must be `S256`. | [required] |
**code_challenge** | **String** | Compute `BASE64URL(SHA256(code_verifier))`. | [required] |
**scope** | Option<**String**> | Space separated list of requested OAuth scopes, if any. |  |
**username** | Option<**String**> | Hint that you want the user to log in with a specific Lichess username. |  |
**state** | Option<**String**> | Arbitrary state that will be returned verbatim with the authorization result. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_test

> ::std::collections::HashMap<String, crate::models::OneOfLessThanGreaterThan> token_test(body)
Test multiple OAuth tokens

For up to 1000 OAuth tokens, returns their associated user ID and scopes, or `null` if the token is invalid.  The method is `POST` so a longer list of tokens can be sent in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | OAuth tokens separated by commas. Up to 1000. | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::OneOfLessThanGreaterThan>**](oneOf<>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

