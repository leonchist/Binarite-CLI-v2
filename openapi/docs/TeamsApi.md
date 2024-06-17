# \TeamsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_team**](TeamsApi.md#create_new_team) | **POST** /teams | 
[**delete_team**](TeamsApi.md#delete_team) | **DELETE** /teams/{id} | 
[**get_all_teams**](TeamsApi.md#get_all_teams) | **GET** /teams | 
[**update_team**](TeamsApi.md#update_team) | **PUT** /teams/{id} | 



## create_new_team

> models::TeamInfo create_new_team(create_team_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_team_request** | [**CreateTeamRequest**](CreateTeamRequest.md) |  | [required] |

### Return type

[**models::TeamInfo**](TeamInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team

> models::TeamInfo delete_team(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The team Id to delete. | [required] |

### Return type

[**models::TeamInfo**](TeamInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_teams

> Vec<models::TeamInfo> get_all_teams(only_active, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_active** | **bool** | The option for active teams | [required] |
**page** | Option<**i32**> | The index of page to show. | [required] |
**per_page** | Option<**i32**> | The number of items to show in one page. | [required] |

### Return type

[**Vec<models::TeamInfo>**](TeamInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> models::TeamInfo update_team(id, update_team_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The team Id to update. | [required] |
**update_team_request** | [**UpdateTeamRequest**](UpdateTeamRequest.md) |  | [required] |

### Return type

[**models::TeamInfo**](TeamInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

