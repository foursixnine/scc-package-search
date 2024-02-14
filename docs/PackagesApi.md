# \PackagesApi

All URIs are relative to */api/package_search*

Method | HTTP request | Description
------------- | ------------- | -------------
[**packages_get**](PackagesApi.md#packages_get) | **GET** /packages | Find sub packages of a base product
[**products_get**](PackagesApi.md#products_get) | **GET** /products | A list of base products that can be searched



## packages_get

> crate::models::PackagesGet200Response packages_get(product_id, accept, query)
Find sub packages of a base product

Warning: This api endpoint is subject to change

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Id of the base product | [required] |
**accept** | [**serde_json::Value**](.md) |  | [required] |[default to application/vnd.scc.suse.com.v4+json]
**query** | Option<**String**> | String to query |  |

### Return type

[**crate::models::PackagesGet200Response**](_packages_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## products_get

> crate::models::ProductsGet200Response products_get(accept)
A list of base products that can be searched

Warning: This api endpoint is subject to change

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | [**serde_json::Value**](.md) |  | [required] |[default to application/vnd.scc.suse.com.v4+json]

### Return type

[**crate::models::ProductsGet200Response**](_products_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

