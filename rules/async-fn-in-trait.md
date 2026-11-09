# async-fn-in-trait

> Prefer native async trait methods for static dispatch; use boxed futures when dynamic dispatch is needed

## Why It Matters

Native `async fn` in traits has been stable since Rust 1.75 and does not require boxing the returned future. Dynamic dispatch and a `Send` future are separate API decisions. `trait-variant` can add `Send` bounds, but does not erase or box the future and does not make an async method dyn-compatible.

## Bad

```rust
#[trait_variant::make(RepoSend: Send)]
trait Repo {
    async fn get(&self, id: u64) -> String;
}

// Still invalid: RepoSend returns an opaque impl Future.
fn use_dyn(_: Box<dyn RepoSend>) {}
```

## Good

```rust
trait Repo {
    async fn get(&self, id: u64) -> String;
}

struct MemoryRepo;

impl Repo for MemoryRepo {
    async fn get(&self, id: u64) -> String {
        format!("row-{id}")
    }
}

async fn load(repo: &impl Repo) -> String {
    repo.get(1).await
}
```

## Dynamic Dispatch

Use `async-trait` on both the trait and implementations, or return `Pin<Box<dyn Future<Output = T> + Send + '_>>` explicitly. Other dyn-compatibility rules still apply. The macro is appropriate when runtime polymorphism is required; it is not inherently an anti-pattern.

```rust
#[async_trait::async_trait]
trait Repo: Send + Sync {
    async fn get(&self, id: u64) -> String;
}

struct MemoryRepo;

#[async_trait::async_trait]
impl Repo for MemoryRepo {
    async fn get(&self, id: u64) -> String {
        format!("row-{id}")
    }
}

fn make_repo() -> Box<dyn Repo> {
    Box::new(MemoryRepo)
}
```

## Send Bounds for Static Dispatch

An abstract native async trait method does not promise that its future is `Send`. Make that promise explicit when callers need to spawn it. For a future that borrows `&self`, the implementation will generally need `Self: Sync` too.

```rust
use std::future::Future;

trait Repo: Send + Sync {
    fn get(&self, id: u64) -> impl Future<Output = String> + Send;
}

struct MemoryRepo;

impl Repo for MemoryRepo {
    async fn get(&self, id: u64) -> String {
        format!("row-{id}")
    }
}

fn spawn_load(repo: impl Repo + 'static) -> tokio::task::JoinHandle<String> {
    tokio::spawn(async move { repo.get(1).await })
}
```

Alternatively, `#[trait_variant::make(RepoSend: Send)]` generates a variant with `Send` bounds for static dispatch. It does not provide the boxed-future adapter shown above.

| Scenario | Approach |
|---|---|
| Static dispatch, no Send requirement | Native `async fn` in trait |
| `dyn Trait` | `async-trait` or explicit boxed futures |
| `tokio::spawn` on any runtime flavor | Explicit `+ Send` or a `trait-variant` Send variant, plus required ownership bounds |
| Direct await or local tasks | Native async methods can return non-Send futures; `spawn_local` requires a local execution context |

## Sources

- [Rust Reference: dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
- [trait-variant](https://docs.rs/trait-variant/latest/trait_variant/)
- [async-trait](https://docs.rs/async-trait/latest/async_trait/)

## See Also

- [trait-object-safety](trait-object-safety.md) - Dyn-compatibility requirements
- [async-async-fn-bounds](async-async-fn-bounds.md) - Higher-order async functions
- [async-tokio-runtime](async-tokio-runtime.md) - Runtime selection
