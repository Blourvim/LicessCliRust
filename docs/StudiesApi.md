# \StudiesApi

All URIs are relative to *https://lichess.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_study_import_pgn**](StudiesApi.md#api_study_import_pgn) | **POST** /api/study/{studyId}/import-pgn | Import PGN into a study
[**study_all_chapters_head**](StudiesApi.md#study_all_chapters_head) | **HEAD** /api/study/{studyId}.pgn | Study metadata
[**study_all_chapters_pgn**](StudiesApi.md#study_all_chapters_pgn) | **GET** /api/study/{studyId}.pgn | Export all chapters
[**study_chapter_pgn**](StudiesApi.md#study_chapter_pgn) | **GET** /api/study/{studyId}/{chapterId}.pgn | Export one study chapter
[**study_export_all_pgn**](StudiesApi.md#study_export_all_pgn) | **GET** /study/by/{username}/export.pgn | Export all studies of a user
[**study_list_metadata**](StudiesApi.md#study_list_metadata) | **GET** /api/study/by/{username} | List studies of a user



## api_study_import_pgn

> serde_json::Value api_study_import_pgn(study_id, name, pgn, orientation, variant)
Import PGN into a study

Imports arbitrary PGN into an existing [study](https://lichess.org/study). Creates a new chapter in the study.  If the PGN contains multiple games (separated by 2 or more newlines) then multiple chapters will be created within the study.  Note that a study can contain at most 64 chapters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**study_id** | **String** | ID of the study | [required] |
**name** | **String** | Name of the new chapter. If multiple chapters are created, the names will be infered from the PGN tags.  | [required] |
**pgn** | **String** | PGN to import. Can contain multiple games separated by 2 or more newlines.  | [required] |
**orientation** | Option<**String**> | Default board orientation. |  |[default to white]
**variant** | Option<[**crate::models::VariantKey**](VariantKey.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## study_all_chapters_head

> study_all_chapters_head(study_id)
Study metadata

Only get the study headers, including `Last-Modified`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**study_id** | **String** | The study ID (8 characters). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## study_all_chapters_pgn

> serde_json::Value study_all_chapters_pgn(study_id, clocks, comments, variations, source, orientation)
Export all chapters

Download all chapters of a study in PGN format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**study_id** | **String** | The study ID (8 characters). | [required] |
**clocks** | Option<**bool**> | Include clock comments in the PGN moves, when available.  Example: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  |  |[default to true]
**comments** | Option<**bool**> | Include analysis and annotator comments in the PGN moves, when available.  Example: `12. Bxf6 { [%eval 0.23] } a3 { White is in a pickle. }`  |  |[default to true]
**variations** | Option<**bool**> | Include non-mainline moves, when available.  Example: `4. d4 Bb4+ (4... Nc6 5. Nf3 Bb4+ 6. Bd2 (6. Nbd2 O-O 7. O-O) 6... Bd6) 5. Nd2`  |  |[default to true]
**source** | Option<**bool**> | Add a `Source` PGN tag with the study chapter URL.  Example: `[Source \"https://lichess.org/study/4NBHImfM/1Tk4IyTz\"]`  |  |[default to false]
**orientation** | Option<**bool**> | Add a `Orientation` PGN tag with the chapter predefined orientation.  Example: `[Orientation \"white\"]`  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## study_chapter_pgn

> serde_json::Value study_chapter_pgn(study_id, chapter_id, clocks, comments, variations, source, orientation)
Export one study chapter

Download one study chapter in PGN format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**study_id** | **String** | The study ID (8 characters). | [required] |
**chapter_id** | **String** | The chapter ID (8 characters). | [required] |
**clocks** | Option<**bool**> | Include clock comments in the PGN moves, when available.  Example: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  |  |[default to true]
**comments** | Option<**bool**> | Include analysis and annotator comments in the PGN moves, when available.  Example: `12. Bxf6 { [%eval 0.23] } a3 { White is in a pickle. }`  |  |[default to true]
**variations** | Option<**bool**> | Include non-mainline moves, when available.  Example: `4. d4 Bb4+ (4... Nc6 5. Nf3 Bb4+ 6. Bd2 (6. Nbd2 O-O 7. O-O) 6... Bd6) 5. Nd2`  |  |[default to true]
**source** | Option<**bool**> | Add a `Source` PGN tag with the study chapter URL.  Example: `[Source \"https://lichess.org/study/4NBHImfM/1Tk4IyTz\"]`  |  |[default to false]
**orientation** | Option<**bool**> | Add a `Orientation` PGN tag with the chapter predefined orientation.  Example: `[Orientation \"white\"]`  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## study_export_all_pgn

> serde_json::Value study_export_all_pgn(username, clocks, comments, variations, source, orientation)
Export all studies of a user

Download all chapters of all studies of a user in PGN format.  If authenticated, then all public, unlisted, and private studies are included.  If not, only public (non-unlisted) studies are included. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The user whose studies we export | [required] |
**clocks** | Option<**bool**> | Include clock comments in the PGN moves, when available.  Example: `2. exd5 { [%clk 1:01:27] } e5 { [%clk 1:01:28] }`  |  |[default to true]
**comments** | Option<**bool**> | Include analysis and annotator comments in the PGN moves, when available.  Example: `12. Bxf6 { [%eval 0.23] } a3 { White is in a pickle. }`  |  |[default to true]
**variations** | Option<**bool**> | Include non-mainline moves, when available.  Example: `4. d4 Bb4+ (4... Nc6 5. Nf3 Bb4+ 6. Bd2 (6. Nbd2 O-O 7. O-O) 6... Bd6) 5. Nd2`  |  |[default to true]
**source** | Option<**bool**> | Add a `Source` PGN tag with the study chapter URL.  Example: `[Source \"https://lichess.org/study/4NBHImfM/1Tk4IyTz\"]`  |  |[default to false]
**orientation** | Option<**bool**> | Add a `Orientation` PGN tag with the chapter predefined orientation.  Example: `[Orientation \"white\"]`  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-chess-pgn

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## study_list_metadata

> Vec<serde_json::Value> study_list_metadata(username)
List studies of a user

Get metadata (name and dates) of all studies of a user.  If authenticated, then all public, unlisted, and private studies are included.  If not, only public (non-unlisted) studies are included.  Studies are streamed as [ndjson](#section/Introduction/Streaming-with-ND-JSON). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The user whose studies we list | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

