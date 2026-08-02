# Jules API Compatibility Report

## Overview
This report compares the current Jules-SDK API implementation against the official Jules API REST documentation at `https://developers.google.com/jules/api/reference/rest`.

## Current Implementation State
Currently, the Jules-SDK is structured with traits and basic abstractions (e.g., `Client`, `HttpRequest`, `Endpoint`, `AuthType`). However, it lacks full structural alignment with the official Google Jules API endpoints.

## Missing Resources & Methods (v1alpha)
The official REST API has the following resource hierarchy under `v1alpha`, which is not fully reflected in the SDK models and endpoints:

### 1. `sessions` Resource
- **POST `/v1alpha/{session=sessions/*}:approvePlan`**: Missing `approvePlan` method/builder.
- **POST `/v1alpha/sessions`**: Missing dedicated session creation method matching this payload.
- **GET `/v1alpha/{name=sessions/*}`**: Missing session retrieval method.
- **GET `/v1alpha/sessions`**: Missing session list method.
- **POST `/v1alpha/{session=sessions/*}:sendMessage`**: Missing specific `sendMessage` method tied to session resource.

### 2. `sessions.activities` Resource
Currently, there are no `Activity` models or abstractions in the SDK.
- **GET `/v1alpha/{name=sessions/*/activities/*}`**: Missing get activity method.
- **GET `/v1alpha/{parent=sessions/*}/activities`**: Missing list activities method.

### 3. `sources` Resource
Currently, there are no `Source` models or abstractions in the SDK.
- **GET `/v1alpha/{name=sources/**}`**: Missing get source method.
- **GET `/v1alpha/sources`**: Missing list sources method.

## SDK Structure Gaps
1. **Resource Path Alignment**: The SDK's `Endpoint` builder (`crates/jules-api/src/http/endpoint.rs`) is generic, but there are no concrete definitions for the `v1alpha` paths (e.g., `/v1alpha/sessions`).
2. **Base URL**: The official service endpoint is `https://jules.googleapis.com`. The SDK needs a configuration default for this.
3. **Models**: The SDK has basic `Session`, `Message`, and `Conversation` structs, but they do not match the exact JSON schema required by Google's REST APIs (e.g., missing specific Google Cloud API request/response structs for `sendMessage` and `activities`).
4. **Authentication**: The SDK has an `AuthType` abstraction, but needs to be configured to support Google's authentication mechanisms (OAuth2/ADC).

## Recommendations
1. **Define API Resource Models**: Create strongly typed Request/Response structs in `jules-core` for `sessions`, `activities`, and `sources` matching the REST schema.
2. **Implement API Clients**: Create specific trait extensions or methods in `jules-api` (e.g., `SessionClient`, `ActivityClient`, `SourceClient`) that map to the v1alpha REST methods.
3. **Update Endpoint Paths**: Define constants for the Google Jules API paths.
4. **Support Google Auth**: Implement Google Application Default Credentials (ADC) or OAuth2 flows as an `AuthType` variant.
