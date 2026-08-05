## 2024-05-18 - Prevent Credential Leakage in Debug Logs
**Vulnerability:** Core configuration and authentication types (`Config`, `BuiltClient`, `ClientBuilder`, `AuthType`) used the standard `#[derive(Debug)]` macro. This meant any sensitive credentials (API keys, bearer tokens) stored in these types would be printed in plaintext if the structures were logged or dumped during a panic.
**Learning:** Default Debug derivations are dangerous for structures holding sensitive data. When `tracing` or `log` is used to dump request/response contexts, or during crash reports, these keys can easily leak into persistent storage (e.g., CloudWatch, Splunk) or console outputs.
**Prevention:** Always manually implement `std::fmt::Debug` for types containing sensitive fields, explicitly redacting the values (e.g., using `"***REDACTED***"`).
## 2024-05-20 - Prevent Credential Leakage in HTTP Requests
**Vulnerability:** `HttpRequest` and `HttpResponse` derived `Debug` by default, leaking raw headers like `Authorization` or `x-api-key` to logs or console outputs.
**Learning:** Default Debug derivations are dangerous for structures holding network request context which often contains sensitive tokens or session cookies.
**Prevention:** Always manually implement `std::fmt::Debug` for HTTP request/response types to explicitly redact sensitive headers like `Authorization`, `x-api-key`, and `Set-Cookie`.
