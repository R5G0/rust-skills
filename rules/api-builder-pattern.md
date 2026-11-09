# api-builder-pattern

> Use Builder pattern for complex construction

## Why It Matters

When a type has many optional parameters or complex initialization, the Builder pattern provides a clear, flexible API. It avoids constructors with many parameters (which are error-prone) and makes the code self-documenting.

## Bad

```rust
// Constructor with many parameters - hard to read, easy to get wrong
let client = Client::new(
    "https://api.example.com",  // Which is which?
    30,                          // Timeout? Retries?
    true,                        // What does this mean?
    None,
    Some("auth_token"),
    false,
);

// Or many Option fields
struct Client {
    url: String,
    timeout: Option<Duration>,
    retries: Option<u32>,
    // ... 10 more optional fields
}
```

## Good

```rust
use std::time::Duration;

pub struct Client { base_url: String, timeout: Duration, max_retries: u32, auth_token: Option<String> }
#[derive(Debug, thiserror::Error)]
pub enum BuilderError { #[error("missing base url")] MissingBaseUrl }

#[must_use = "builders do nothing unless you call build()"]
pub struct ClientBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: u32,
    auth_token: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self { Self { base_url: None, timeout: None, max_retries: 3, auth_token: None } }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the base URL for all requests.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }
    
    /// Sets the request timeout. Default is 30 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Sets the maximum number of retries. Default is 3.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
    
    /// Sets the authentication token.
    pub fn auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }
    
    /// Builds the client with the configured options.
    pub fn build(self) -> Result<Client, BuilderError> {
        let base_url = self.base_url
            .ok_or(BuilderError::MissingBaseUrl)?;
        
        Ok(Client {
            base_url,
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            max_retries: self.max_retries,
            auth_token: self.auth_token,
        })
    }
}

// Usage - clear and self-documenting
let client = ClientBuilder::new()
    .base_url("https://api.example.com")
    .timeout(Duration::from_secs(10))
    .max_retries(5)
    .auth_token("secret")
    .build()?;
```

## Builder Variations

```rust
// 1. Infallible builder (build() returns T, not Result)
impl WidgetBuilder {
    pub fn build(self) -> Widget {
        Widget {
            color: self.color.unwrap_or(Color::Black),
            size: self.size.unwrap_or(Size::Medium),
        }
    }
}

// 2. Typestate builder (compile-time required field checking)
pub struct ClientBuilder<Url> {
    url: Url,
    timeout: Option<Duration>,
}

pub struct NoUrl;
pub struct HasUrl(String);

impl ClientBuilder<NoUrl> {
    pub fn new() -> Self {
        Self { url: NoUrl, timeout: None }
    }
    
    pub fn url(self, url: String) -> ClientBuilder<HasUrl> {
        ClientBuilder { url: HasUrl(url), timeout: self.timeout }
    }
}

impl ClientBuilder<HasUrl> {
    pub fn build(self) -> Client {
        // url is guaranteed to be set
        Client { url: self.url.0, timeout: self.timeout }
    }
}

// 3. Consuming vs borrowing (consuming is more common)
struct ConsumingOptions { timeout: Duration }
impl ConsumingOptions {
    pub fn timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
}
struct BorrowingOptions { timeout: Duration }
impl BorrowingOptions {
    pub fn timeout(&mut self, t: Duration) -> &mut Self { self.timeout = t; self }
}
```

## Evidence from reqwest

```rust
// https://github.com/seanmonstar/reqwest/blob/master/src/async_impl/client.rs

#[must_use]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            config: Config::default(),
        }
    }
    
    pub fn timeout(mut self, timeout: Duration) -> ClientBuilder {
        self.config.timeout = Some(timeout);
        self
    }
    
    pub fn build(self) -> Result<Client, Error> {
        // Validation and construction
    }
}
```

## Key Attributes

```rust
#[derive(Default)]  // Enables MyBuilder::default()
#[must_use = "builders do nothing unless you call build()"]
pub struct MyBuilder { ... }

impl MyBuilder {
    #[must_use]  // Each method should have this
    pub fn option(mut self, value: T) -> Self { ... }
}
```

## Construction Contracts

Group parameters that already form a meaningful domain value, and keep the order of repeated conceptual parameters consistent across related functions. Named fields and validated types reduce mix-ups without requiring an arbitrary parameter-count threshold or a generic dependency container.

For a fallible builder, validate cross-field constraints in build and return one Result. Setters can remain simple configuration steps; an input type may validate its own invariant before it reaches the builder. Early failure remains appropriate when a setter performs a real fallible operation or must reject an unsafe state immediately.

Choose required constructor parameters, typestate, or final validation according to the actual contract. Do not add a builder solely because a fixed number of options was exceeded, or change a published constructor just to match a naming preference. Put callbacks near the end where practical, while preserving established APIs that legitimately need multiple callbacks.

Sources: [Microsoft M-INIT-BUILDER](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-INIT-BUILDER), [M-INIT-CASCADED](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-INIT-CASCADED), [M-BUILD-RESULT](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-BUILD-RESULT), [M-PARAMETER-CONSISTENCY](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-PARAMETER-CONSISTENCY).

## Verified Example

<!-- verify: compile -->
```rust
use std::time::Duration;

#[derive(Debug, PartialEq)]
struct Client { base_url: String, timeout: Duration }
#[derive(Default)]
#[must_use = "call build to create the client"]
struct ClientBuilder { base_url: Option<String>, timeout: Option<Duration> }
impl ClientBuilder {
    fn base_url(mut self, url: impl Into<String>) -> Self { self.base_url = Some(url.into()); self }
    fn timeout(mut self, timeout: Duration) -> Self { self.timeout = Some(timeout); self }
    fn build(self) -> Result<Client, &'static str> {
        Ok(Client { base_url: self.base_url.ok_or("missing base url")?, timeout: self.timeout.unwrap_or(Duration::from_secs(30)) })
    }
}
fn main() {
    assert_eq!(ClientBuilder::default().build(), Err("missing base url"));
    let client = ClientBuilder::default().base_url("https://example.com").timeout(Duration::from_secs(5)).build().unwrap();
    assert_eq!(client.timeout, Duration::from_secs(5));
}
```

## See Also

- [api-builder-must-use](api-builder-must-use.md) - Add #[must_use] to builders
- [api-typestate](api-typestate.md) - Compile-time state machines
- [api-impl-into](api-impl-into.md) - Accept impl Into for flexibility
