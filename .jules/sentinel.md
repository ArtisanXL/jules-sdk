## 2024-05-18 - Prevent Credential Leakage in Debug Logs
**Vulnerability:** Core configuration and authentication types (`Config`, `BuiltClient`, `ClientBuilder`, `AuthType`) used the standard `#[derive(Debug)]` macro. This meant any sensitive credentials (API keys, bearer tokens) stored in these types would be printed in plaintext if the structures were logged or dumped during a panic.
**Learning:** Default Debug derivations are dangerous for structures holding sensitive data. When `tracing` or `log` is used to dump request/response contexts, or during crash reports, these keys can easily leak into persistent storage (e.g., CloudWatch, Splunk) or console outputs.
**Prevention:** Always manually implement `std::fmt::Debug` for types containing sensitive fields, explicitly redacting the values (e.g., using `"***REDACTED***"`).

## 2025-02-28 - Prevent Credential Leakage in HTTP Logs
**Vulnerability:** HTTP abstraction structs (`HttpRequest` and `HttpResponse`) used the standard `#[derive(Debug)]` macro. This meant any sensitive credentials (API keys, bearer tokens) passed as HTTP headers would be printed in plaintext if the structures were logged or dumped during a panic.
**Learning:** Extending the previous learning, even low-level transport types must carefully avoid default Debug derivations. Data passing through transport abstractions is often logged in aggregate, meaning transport structs are prime targets for unintended data leakage.
**Prevention:** Removed `#[derive(Debug)]` and manually implemented `std::fmt::Debug` using a helper `RedactedHeaders` struct to safely mask known sensitive header values (like `Authorization`, `x-api-key`, and `Set-Cookie`).
