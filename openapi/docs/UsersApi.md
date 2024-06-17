# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /users | 
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{id} | 
[**get_all_users**](UsersApi.md#get_all_users) | **GET** /users | 
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{id} | 



## create_user

> models::UserInfo create_user(create_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::UserInfo**](UserInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::DeleteUserResponse delete_user(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The user Id to delete. | [required] |

### Return type

[**models::DeleteUserResponse**](DeleteUserResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users

> models::UserInfo get_all_users(isview, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**isview** | **bool** | The option for operating users | [required] |
**page** | Option<**i32**> | The index of page to show. | [required] |
**per_page** | Option<**i32**> | The number of items to show in one page. | [required] |

### Return type

[**models::UserInfo**](UserInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::UpdateUserResponse update_user(id, update_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The user Id to update. | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**models::UpdateUserResponse**](UpdateUserResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

