# Rust Skills

![rules](https://img.shields.io/badge/rules-273-blue)
![categories](https://img.shields.io/badge/categories-26-blue)
![Rust](https://img.shields.io/badge/Rust-1.97.1%20%2F%202024%20edition-orange)
![license](https://img.shields.io/badge/license-MIT-green)

273 Rust rules your AI coding agent can use to write better code. Current for Rust 1.97.1 (2024 edition).

Works with Claude Code, Cursor, Windsurf, Copilot, Codex, Aider, Zed, Amp, Cline, and pretty much any other agent that supports skills.

This fork fixes reviewed upstream issues and adds selected portable Google guidance. See [the source review](references/source-review.md) for scope and applicability.

## Why

Out of the box, coding agents write *average* Rust — they clone to dodge the borrow checker, `.unwrap()` everything, and reach for `Box<dyn Trait>` when `impl Trait` would do. These rules encode what expert Rust actually looks like: idiomatic, fast, and safe. Each rule is small and focused, so the agent pulls in only what's relevant to the code in front of it.

## Install

```bash
npx skills add R5G0/rust-skills
```

That's it. The CLI figures out which agents you have and installs the skill to the right place.

## How to use it

After installing, just ask your agent:

```
/rust-skills review this function
```

```
/rust-skills is my error handling idiomatic?
```

```
/rust-skills check for memory issues
```

```
/rust-skills is this unsafe block sound?
```

The agent loads the relevant rules and applies them to your code.

## See it in action

Ask the agent to review a function like this:

```rust
// before
fn first_word(s: &String) -> String {
    s.clone().split_whitespace().next().unwrap().to_string()
}
```

With these rules loaded, it knows to take `&str` instead of `&String`, drop the
needless `clone()` and allocation, and return an `Option` instead of panicking:

```rust
// after — applies own-slice-over-vec, own-borrow-over-clone, anti-unwrap-abuse
fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}
```

## What's in here

273 rules split into 26 categories:

| Category | Rules | What it covers |
|----------|-------|----------------|
| **Ownership & Borrowing** | 12 | When to borrow vs clone, Arc/Rc, lifetimes |
| **Error Handling** | 12 | thiserror for libs, anyhow for apps, the `?` operator |
| **Memory** | 17 | SmallVec, arenas, avoiding allocations, `mem::take`, drop order |
| **Unsafe Code** | 7 | `SAFETY:` comments, Miri, `MaybeUninit`, 2024-edition unsafe |
| **API Design** | 17 | Builder pattern, newtypes, sealed traits, `FromIterator` |
| **Async** | 18 | Tokio patterns, channels, async fn in traits, cancel safety |
| **Concurrency** | 5 | rayon, scoped threads, atomic ordering, thread-locals |
| **Optimization** | 12 | LTO, inlining, PGO, SIMD |
| **Numeric & Arithmetic** | 6 | Overflow handling, `as` vs `TryFrom`, float compare, `NonZero` |
| **Type Safety** | 15 | Newtypes, parse don't validate, `Deref`, `Display`/`Debug` |
| **Trait & Generics Design** | 7 | dyn vs generic, associated types, blanket impls, object safety, orphan rule |
| **Conversions** | 3 | `TryFrom`, `FromStr`, `AsMut` |
| **Const & Compile-Time** | 4 | `const fn`, const vs static, const generics, `const {}` blocks |
| **Serde** | 8 | rename_all, default, flatten, enum tagging, validate-on-deserialize |
| **Pattern Matching** | 6 | `let-else`, `matches!`, if-let chains, exhaustive matches |
| **Macros** | 8 | `macro_rules!` hygiene, fragment specifiers, proc-macros with syn/quote |
| **Closures** | 5 | Fn/FnMut/FnOnce bounds, returning `impl Fn`, move & disjoint capture |
| **Collections** | 4 | HashMap/BTreeMap/IndexMap, Vec/VecDeque, sets, `BinaryHeap` |
| **Naming** | 16 | Following Rust API Guidelines |
| **Testing** | 15 | Proptest, mockall, criterion, loom, snapshot tests |
| **Docs** | 12 | Doc examples, intra-doc links, README/crate-doc unification |
| **Observability** | 7 | tracing over log, spans, structured fields, redacting secrets |
| **Performance** | 13 | Iterators, entry API, faster hashers, I/O buffering |
| **Project Structure** | 15 | Workspaces, module layout, features, MSRV |
| **Linting** | 14 | Clippy config, CI setup, `unexpected_cfgs` |
| **Anti-patterns** | 15 | Common mistakes and how to fix them |

Each rule has:
- Why it matters
- A bad code example
- A good code example
- Links to related rules and sources

## How it works

The design is built for low token cost and easy auditing:

- **[`SKILL.md`](./SKILL.md)** is a lightweight index — every rule listed as a one-line summary, grouped by category, with a link to its file. The agent reads this first.
- **[`rules/`](./rules)** holds one Markdown file per rule (`<prefix>-<name>.md`). The agent opens only the handful relevant to your code instead of loading the full corpus — progressive disclosure keeps context small.
- **Prefixes** (`own-`, `err-`, `unsafe-`, `async-`, …) map directly to categories, so an agent reviewing async code can pull just `async-`, `conc-`, and `own-` rules.

`CLAUDE.md` and `AGENTS.md` are symlinks to `SKILL.md`, so the same content works across agent conventions.

## Manual install

Install the complete directory so the index can resolve its rule and reference files.

For Codex, Amp, and agents that discover project skills under `.agents/skills`:

```bash
git clone https://github.com/R5G0/rust-skills.git .agents/skills/rust-skills
```

For Claude Code:

```bash
# One project
git clone https://github.com/R5G0/rust-skills.git .claude/skills/rust-skills
# Or all projects
git clone https://github.com/R5G0/rust-skills.git ~/.claude/skills/rust-skills
```

For Cursor:

```bash
git clone https://github.com/R5G0/rust-skills.git .cursor/skills/rust-skills
```

For other agents, clone this repository to a persistent directory and add a
reference to its `SKILL.md` in the existing agent configuration or instruction
file. Preserve the adjacent `rules/` and `references/` directories. A single-file
copy of the index does not include the rules it links to.

For Aider, the cloned entrypoint can be supplied with:

```bash
aider --read path/to/rust-skills/SKILL.md
```

## All rules

See [SKILL.md](./SKILL.md) for the full list with links to each rule file.

## Sources & attribution

These rules are an independent synthesis of official Rust guidance, well-known books, and patterns drawn from widely-used open-source crates. They are not affiliated with or endorsed by the Rust project or any crate author. The text and code examples are original summaries — no substantial content is copied from the sources below.

**Official Rust documentation**
- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe code)
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Standard library docs](https://doc.rust-lang.org/std/) and [release notes](https://doc.rust-lang.org/releases.html)

**Books & guides**
- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) and [Comprehensive Rust](https://google.github.io/comprehensive-rust/) — selected portable guidance, mapped in [the source review](references/source-review.md)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/) — Nicholas Nethercote
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) — rust-unofficial
- [Rust Atomics and Locks](https://marabos.nl/atomics/) — Mara Bos
- [Effective Rust](https://effective-rust.com/) — David Drysdale

**Tooling**
- [Clippy lint documentation](https://rust-lang.github.io/rust-clippy/)
- [Miri](https://github.com/rust-lang/miri)

**Real-world codebases studied for idioms**
- ripgrep, tokio, serde, clap, polars, axum, cargo, hyper, bevy, rayon, and dtolnay's crates (thiserror, anyhow, syn)

This project is MIT-licensed. Referenced upstream materials remain under their own licenses — the official Rust documentation and API Guidelines are dual [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) / [Apache-2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE).

## Contributing

PRs welcome. To add or change a rule:

1. Create `rules/<prefix>-<name>.md` using a `kebab-case` id with an existing category prefix (`own-`, `err-`, `mem-`, …).
2. Follow the format of existing rules: a `>` one-line summary, then `## Why It Matters`, `## Bad`, `## Good`, and `## See Also` (with links that resolve).
3. Make sure code examples compile on current stable Rust.
4. Run `python3 checks/gen_index.py` to update the index and counts, then run the checks in [CONTRIBUTING.md](CONTRIBUTING.md).

````markdown
# prefix-rule-name

> One-line imperative summary.

## Why It Matters

Two to four sentences.

## Bad

```rust
// the anti-pattern
```

## Good

```rust
// the recommended pattern
```

## See Also

- [other-rule](other-rule.md) - why it's related
````

## License

MIT
