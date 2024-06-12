# \MetaspheresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_metasphere**](MetaspheresApi.md#create_metasphere) | **POST** /projects/{project_id}/metaspheres | 
[**delete_metasphere**](MetaspheresApi.md#delete_metasphere) | **DELETE** /metaspheres/{metasphere_id} | 
[**get_metasphere**](MetaspheresApi.md#get_metasphere) | **GET** /metaspheres/{metasphere_id} | 
[**get_metasphere_outputs**](MetaspheresApi.md#get_metasphere_outputs) | **GET** /metaspheres/{metasphere_id}/outputs | 
[**get_metasphere_status**](MetaspheresApi.md#get_metasphere_status) | **GET** /metaspheres/{metasphere_id}/status | 
[**get_metaspheres_from_project**](MetaspheresApi.md#get_metaspheres_from_project) | **GET** /projects/{project_id}/metaspheres/ | 



## create_metasphere

> models::CreateMetasphereResponse create_metasphere(project_id, create_metasphere_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | Id of the project the metasphere belongs to. | [required] |
**create_metasphere_request** | [**CreateMetasphereRequest**](CreateMetasphereRequest.md) |  | [required] |

### Return type

[**models::CreateMetasphereResponse**](CreateMetasphereResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metasphere

> delete_metasphere(metasphere_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metasphere_id** | **i32** | Id of the metasphere to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metasphere

> models::MetasphereDb get_metasphere(metasphere_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metasphere_id** | **i32** | Id of the metasphere to get the details from. | [required] |

### Return type

[**models::MetasphereDb**](MetasphereDB.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metasphere_outputs

> models::MetasphereOutputs get_metasphere_outputs(metasphere_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metasphere_id** | **i32** | Id of the metasphere to get the outputs from. | [required] |

### Return type

[**models::MetasphereOutputs**](MetasphereOutputs.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metasphere_status

> models::MetasphereStatusResponse get_metasphere_status(metasphere_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metasphere_id** | **i32** | Id of the metasphere to get the status from. | [required] |

### Return type

[**models::MetasphereStatusResponse**](MetasphereStatusResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metaspheres_from_project

> Vec<models::MetasphereDb> get_metaspheres_from_project(project_id, show_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | Id of the project to retrieve the metasphere from. | [required] |
**show_deleted** | Option<**bool**> | Include Deleted metaspheres in the result. (false if ommited) |  |

### Return type

[**Vec<models::MetasphereDb>**](MetasphereDB.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

