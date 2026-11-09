# opt-target-cpu

> Target only CPU features guaranteed by the deployment fleet; measure before raising the baseline

## Why It Matters

A CPU-specific build can use instructions unavailable on older machines. The build host is not necessarily representative of the deployment environment: globally enabling its features can cause illegal-instruction failures elsewhere. Use an explicit supported fleet baseline for distributed applications, and benchmark the benefit. Generic defaults are appropriate for portable binaries.

## Bad

```toml
# Shared CI configuration: the resulting binary is deployed to unknown CPUs.
[build]
rustflags = ["-C", "target-cpu=native"]
```

## Good

```toml
# .cargo/config.toml for an application fleet guaranteed to support x86-64-v3.
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=x86-64-v3"]
```

Select this configuration only after verifying every production host, failover
node, and test environment meets the same minimum. `native` is appropriate for
local measurements or controlled same-CPU deployments, not a portable default.

## Runtime Dispatch

For portable builds, keep the general baseline and dispatch selected hot paths
after detecting the required features. Avoid compiling the entire binary with
those features: code outside the dispatch can then use them too.

```rust
fn sum_bytes(data: &[u8]) -> u64 {
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("avx2") {
        // SAFETY: runtime detection verified the function's target feature.
        return unsafe { sum_avx2(data) };
    }
    sum_generic(data)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn sum_avx2(data: &[u8]) -> u64 {
    // Illustrative dispatch boundary, not a promise of vectorization.
    sum_generic(data)
}

fn sum_generic(data: &[u8]) -> u64 {
    data.iter().map(|&value| u64::from(value)).sum()
}
```

Inspect generated code and measure realistic input sizes. Enabling AVX2 does not
guarantee a scalar algorithm will vectorize, and SIMD extensions do not imply
that unrelated features such as FMA or BMI are enabled.

## Sources

- [Microsoft M-TARGET-CPU](https://microsoft.github.io/rust-guidelines/guidelines/apps/index.html#M-TARGET-CPU)
- [rustc target-cpu](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu)

## See Also

- [opt-simd-portable](opt-simd-portable.md) - Stable versus nightly SIMD options
- [perf-profile-first](perf-profile-first.md) - Measure actual hot paths
