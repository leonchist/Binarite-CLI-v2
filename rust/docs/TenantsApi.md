# \TenantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantsApi.md#create_tenant) | **POST** /tenants | 
[**delete_tenant**](TenantsApi.md#delete_tenant) | **DELETE** /tenants/{id} | 
[**get_all_tenants**](TenantsApi.md#get_all_tenants) | **GET** /tenants | 
[**update_tenant**](TenantsApi.md#update_tenant) | **PUT** /tenants/{id} | 



## create_tenant

> Vec<models::TenantInfo> create_tenant(name, website)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Tenant name | [required] |
**website** | **String** | Tenant website | [required] |

### Return type

[**Vec<models::TenantInfo>**](TenantInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> models::TenantInfo delete_tenant(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The tenant Id to delete. | [required] |

### Return type

[**models::TenantInfo**](TenantInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tenants

> models::TenantInfo get_all_tenants(page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The index of page to show. | [required] |
**per_page** | Option<**i32**> | The number of items to show in one page. | [required] |

### Return type

[**models::TenantInfo**](TenantInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant

> models::TenantInfo update_tenant(id, name, website)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The tenant Id to update. | [required] |
**name** | **String** | Tenant name | [required] |
**website** | **String** | Tenant website | [required] |

### Return type

[**models::TenantInfo**](TenantInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

