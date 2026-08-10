## 2024-05-18 - Prevent Credential Leakage in Debug Logs
**Vulnerability:** Core configuration and authentication types (`Config`, `BuiltClient`, `ClientBuilder`, `AuthType`) used the standard `#[derive(Debug)]` macro. This meant any sensitive credentials (API keys, bearer tokens) stored in these types would be printed in plaintext if the structures were logged or dumped during a panic.
**Learning:** Default Debug derivations are dangerous for structures holding sensitive data. When `tracing` or `log` is used to dump request/response contexts, or during crash reports, these keys can easily leak into persistent storage (e.g., CloudWatch, Splunk) or console outputs.
**Prevention:** Always manually implement `std::fmt::Debug` for types containing sensitive fields, explicitly redacting the values (e.g., using `"***REDACTED***"`).
## 2024-05-20 - Prevent Credential Leakage in HTTP Requests
**Vulnerability:** `HttpRequest` and `HttpResponse` derived `Debug` by default, leaking raw headers like `Authorization` or `x-api-key` to logs or console outputs.
**Learning:** Default Debug derivations are dangerous for structures holding network request context which often contains sensitive tokens or session cookies.
**Prevention:** Always manually implement `std::fmt::Debug` for HTTP request/response types to explicitly redact sensitive headers like `Authorization`, `x-api-key`, and `Set-Cookie`.
## 2024-05-24 - Prevent HTTP Header CRLF Injection
**Vulnerability:** `HttpRequest::with_header` accepted arbitrary strings for header keys and values without sanitizing CRLF (`\r\n`) characters, leading to HTTP Header Injection (CRLF Injection).
**Learning:** Always sanitize inputs that become part of HTTP headers. Unvalidated headers can allow attackers to inject custom headers or manipulate the HTTP request.
**Prevention:** Added sanitization to explicitly strip `\r` and `\n` characters from keys and values in `HttpRequest::with_header`.
## 2024-08-09 - Prevent Insecure Config File Permissions
**Vulnerability:** The CLI configuration file (which contains the user's sensitive Jules API key) was being created using `std::fs::write`, which defaults to 0644 permissions (-rw-r--r--). This allowed any other local user on the system to read the API key.
**Learning:** Default file creation modes do not restrict read access to the file owner. When writing sensitive credentials to the filesystem, explicit permissions must be set to restrict access.
**Prevention:** Use `std::fs::OpenOptions` combined with `std::os::unix::fs::OpenOptionsExt`'s `.mode(0o600)` to ensure sensitive configuration files are created with read/write access restricted solely to the owner.
## 2024-08-10 - Prevent Credential Leakage in CLI Config Debug Logs
**Vulnerability:** The `CliConfig` struct derived `Debug` by default, meaning any logs or console outputs dumping the CLI configuration would leak the user's `api_key` in plaintext.
**Learning:** Automatically derived `Debug` implementations on structures storing configuration secrets are a primary source of credential leakage, especially in CLI tools where developers might dump config state for debugging.
**Prevention:** Manually implement `std::fmt::Debug` for `CliConfig` (and similar structures) to explicitly redact sensitive fields like `api_key` using `"***REDACTED***"`.
