# \ProjectsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_project**](ProjectsApi.md#create_new_project) | **POST** /projects | 
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /projects/{id} | 
[**get_all_projects**](ProjectsApi.md#get_all_projects) | **GET** /projects | 
[**update_project**](ProjectsApi.md#update_project) | **PUT** /projects/{id} | 



## create_new_project

> models::ProjectInfo create_new_project(create_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request** | [**CreateProjectRequest**](CreateProjectRequest.md) |  | [required] |

### Return type

[**models::ProjectInfo**](ProjectInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> models::ProjectInfo delete_project(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The project Id to delete. | [required] |

### Return type

[**models::ProjectInfo**](ProjectInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_projects

> Vec<models::ProjectInfo> get_all_projects(only_active, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_active** | **bool** | The option for active projects | [required] |
**page** | Option<**i32**> | The index of page to show. | [required] |
**per_page** | Option<**i32**> | The number of items to show in one page. | [required] |

### Return type

[**Vec<models::ProjectInfo>**](ProjectInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::ProjectInfo update_project(id, update_project_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The project Id to update. | [required] |
**update_project_request** | [**UpdateProjectRequest**](UpdateProjectRequest.md) |  | [required] |

### Return type

[**models::ProjectInfo**](ProjectInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

