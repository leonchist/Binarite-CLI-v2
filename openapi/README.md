# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 0.1.0
- Generator version: 7.7.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*MetaspheresApi* | [**create_metasphere**](docs/MetaspheresApi.md#create_metasphere) | **POST** /projects/{project_id}/metaspheres | 
*MetaspheresApi* | [**delete_metasphere**](docs/MetaspheresApi.md#delete_metasphere) | **DELETE** /metaspheres/{metasphere_id} | 
*MetaspheresApi* | [**get_metasphere**](docs/MetaspheresApi.md#get_metasphere) | **GET** /metaspheres/{metasphere_id} | 
*MetaspheresApi* | [**get_metasphere_outputs**](docs/MetaspheresApi.md#get_metasphere_outputs) | **GET** /metaspheres/{metasphere_id}/outputs | 
*MetaspheresApi* | [**get_metasphere_status**](docs/MetaspheresApi.md#get_metasphere_status) | **GET** /metaspheres/{metasphere_id}/status | 
*MetaspheresApi* | [**get_metaspheres_from_project**](docs/MetaspheresApi.md#get_metaspheres_from_project) | **GET** /projects/{project_id}/metaspheres/ | 
*ProjectsApi* | [**create_new_project**](docs/ProjectsApi.md#create_new_project) | **POST** /projects | 
*ProjectsApi* | [**delete_project**](docs/ProjectsApi.md#delete_project) | **DELETE** /projects/{id} | 
*ProjectsApi* | [**get_all_projects**](docs/ProjectsApi.md#get_all_projects) | **GET** /projects | 
*ProjectsApi* | [**update_project**](docs/ProjectsApi.md#update_project) | **PUT** /projects/{id} | 
*TeamsApi* | [**create_new_team**](docs/TeamsApi.md#create_new_team) | **POST** /teams | 
*TeamsApi* | [**delete_team**](docs/TeamsApi.md#delete_team) | **DELETE** /teams/{id} | 
*TeamsApi* | [**get_all_teams**](docs/TeamsApi.md#get_all_teams) | **GET** /teams | 
*TeamsApi* | [**update_team**](docs/TeamsApi.md#update_team) | **PUT** /teams/{id} | 
*TenantsApi* | [**create_tenant**](docs/TenantsApi.md#create_tenant) | **POST** /tenants | 
*TenantsApi* | [**delete_tenant**](docs/TenantsApi.md#delete_tenant) | **DELETE** /tenants/{id} | 
*TenantsApi* | [**get_all_tenants**](docs/TenantsApi.md#get_all_tenants) | **GET** /tenants | 
*TenantsApi* | [**update_tenant**](docs/TenantsApi.md#update_tenant) | **PUT** /tenants/{id} | 
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **POST** /users | 
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /users/{id} | 
*UsersApi* | [**get_all_users**](docs/UsersApi.md#get_all_users) | **GET** /users | 
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **PUT** /users/{id} | 


## Documentation For Models

 - [CreateMetasphereRequest](docs/CreateMetasphereRequest.md)
 - [CreateMetasphereResponse](docs/CreateMetasphereResponse.md)
 - [CreateProjectRequest](docs/CreateProjectRequest.md)
 - [CreateTeamRequest](docs/CreateTeamRequest.md)
 - [CreateUserRequest](docs/CreateUserRequest.md)
 - [CreateUserResponse](docs/CreateUserResponse.md)
 - [DeleteUserResponse](docs/DeleteUserResponse.md)
 - [InstanceSize](docs/InstanceSize.md)
 - [MetasphereDb](docs/MetasphereDb.md)
 - [MetasphereOutputs](docs/MetasphereOutputs.md)
 - [MetasphereStatus](docs/MetasphereStatus.md)
 - [MetasphereStatusResponse](docs/MetasphereStatusResponse.md)
 - [ProjectInfo](docs/ProjectInfo.md)
 - [ProjectManagementResponse](docs/ProjectManagementResponse.md)
 - [ResponseStatus](docs/ResponseStatus.md)
 - [Roles](docs/Roles.md)
 - [TeamInfo](docs/TeamInfo.md)
 - [TeamManagementResponse](docs/TeamManagementResponse.md)
 - [TenantInfo](docs/TenantInfo.md)
 - [TenantManagementResponse](docs/TenantManagementResponse.md)
 - [UpdateProjectRequest](docs/UpdateProjectRequest.md)
 - [UpdateTeamRequest](docs/UpdateTeamRequest.md)
 - [UpdateUserRequest](docs/UpdateUserRequest.md)
 - [UpdateUserResponse](docs/UpdateUserResponse.md)
 - [UserInfo](docs/UserInfo.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



