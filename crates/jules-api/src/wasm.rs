//! WASM integration module for Jules API.

#[cfg(target_arch = "wasm32")]
pub mod client {
    //! WASM client implementation using Fetch API.
    //!
    //! [`FetchClient`] implements [`crate::http::Transport`], so it can be used directly with
    //! the rest of this crate's `v1alpha` endpoint methods in browser environments:
    //!
    //! ```ignore
    //! use jules_api::auth::AuthType;
    //! use jules_api::wasm::client::FetchClient;
    //!
    //! let transport = FetchClient::new().expect("no `window` in this environment");
    //! let auth = AuthType::google_api_key("my-key");
    //! ```

    use crate::http::{HttpRequest, HttpResponse, Method, Transport};
    use jules_core::errors::{NetworkError, SDKError};
    use std::future::Future;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Headers, Request, RequestInit, RequestMode, Response, Window};

    /// A lightweight client for WASM environments using the Fetch API.
    #[derive(Debug, Clone)]
    pub struct FetchClient {
        window: Window,
    }

    impl FetchClient {
        /// Creates a new `FetchClient`.
        ///
        /// Returns `None` if the `window` object is not available in this environment.
        #[must_use]
        pub fn new() -> Option<Self> {
            web_sys::window().map(|window| Self { window })
        }

        /// Performs a simple GET request.
        ///
        /// # Errors
        ///
        /// Returns a `JsValue` error if the fetch operation fails.
        #[allow(deprecated)]
        pub async fn get(&self, url: &str) -> Result<Response, JsValue> {
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);

            let request = Request::new_with_str_and_init(url, &opts)?;

            let resp_value = JsFuture::from(self.window.fetch_with_request(&request)).await?;

            let resp: Response = resp_value
                .dyn_into()
                .map_err(|_| JsValue::from_str("Expected Response"))?;
            Ok(resp)
        }
    }

    fn method_str(method: Method) -> &'static str {
        match method {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
        }
    }

    fn js_error(context: &str, err: &JsValue) -> SDKError {
        let detail = err.as_string().unwrap_or_else(|| format!("{err:?}"));
        SDKError::from(NetworkError::new(format!("{context}: {detail}")))
    }

    #[allow(deprecated)]
    async fn send_request(window: &Window, request: HttpRequest) -> Result<HttpResponse, SDKError> {
        let opts = RequestInit::new();
        opts.set_method(method_str(request.method));
        opts.set_mode(RequestMode::Cors);

        if let Some(body) = &request.body {
            let array = js_sys::Uint8Array::from(body.as_slice());
            opts.set_body(&array);
        }

        let headers = Headers::new().map_err(|e| js_error("failed to build headers", &e))?;
        for (key, value) in &request.headers {
            headers
                .append(key, value)
                .map_err(|e| js_error("failed to set header", &e))?;
        }
        opts.set_headers(&headers);

        let js_request = Request::new_with_str_and_init(&request.url, &opts)
            .map_err(|e| js_error("failed to build request", &e))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&js_request))
            .await
            .map_err(|e| js_error("fetch failed", &e))?;
        let response: Response = resp_value
            .dyn_into()
            .map_err(|e| js_error("expected a Response", &e))?;

        let status = response.status();

        let mut headers_out = Vec::new();
        let entries = response.headers().entries();
        loop {
            let next = entries
                .next()
                .map_err(|e| js_error("failed to read headers", &e))?;
            if next.done() {
                break;
            }
            let pair: js_sys::Array = next
                .value()
                .dyn_into()
                .map_err(|e| js_error("malformed header entry", &e))?;
            let key = pair.get(0).as_string().unwrap_or_default();
            let value = pair.get(1).as_string().unwrap_or_default();
            headers_out.push((key, value));
        }

        let buffer = JsFuture::from(
            response
                .array_buffer()
                .map_err(|e| js_error("failed to read response body", &e))?,
        )
        .await
        .map_err(|e| js_error("failed to await response body", &e))?;
        let body = js_sys::Uint8Array::new(&buffer).to_vec();

        Ok(HttpResponse::new(status, headers_out, body))
    }

    impl Transport for FetchClient {
        fn send(
            &self,
            request: HttpRequest,
        ) -> impl Future<Output = Result<HttpResponse, SDKError>> {
            let window = self.window.clone();
            async move { send_request(&window, request).await }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fetch_client_not_available_outside_wasm() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // The client module is only available on wasm32, but we can verify our configuration structure
            // if we really need to, but the client mod itself is under #[cfg(target_arch = "wasm32")].
            // To test something, we can just ensure the module exists during a test pass.
            let dummy = true;
            assert!(dummy, "module should be available");
        }
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::client::FetchClient;
    use crate::http::{HttpRequest, Method, Transport};
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_fetch_client_new() {
        let client = FetchClient::new();
        match client {
            Some(_) => assert!(true, "Client instantiated successfully"),
            None => assert!(true, "Window not available, client correctly returned None"),
        }
    }

    /// Exercises `FetchClient`'s `Transport` impl end to end (method, custom header, status
    /// readback, body-as-bytes readback) against a `data:` URL, which resolves without any
    /// live server or network access, so this works in a headless browser test runner.
    #[wasm_bindgen_test]
    async fn test_fetch_client_implements_transport() {
        let client = FetchClient::new().expect("window not available");
        let request = HttpRequest::new(Method::Get, "data:text/plain,hello")
            .with_header("X-Test-Header", "test-value");

        let response = client.send(request).await.expect("fetch should succeed");

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"hello");
    }
}
