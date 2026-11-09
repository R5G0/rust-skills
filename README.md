# Rust Skills

![rules](https://img.shields.io/badge/rules-284-blue)
![categories](https://img.shields.io/badge/categories-26-blue)
![Rust](https://img.shields.io/badge/Rust-1.97.1%20%2F%202024%20edition-orange)
![license](https://img.shields.io/badge/license-MIT-green)

284 Rust rules your AI coding agent can use to write better code. Current for Rust 1.97.1 (2024 edition).

Works with Claude Code, Cursor, Windsurf, Copilot, Codex, Aider, Zed, Amp, Cline, and pretty much any other agent that supports skills.

This fork fixes reviewed upstream issues and adds portable Google and Microsoft guidance. See [issue coverage](references/upstream-issues.md), [the source review](references/source-review.md), and [Microsoft applicability mapping](references/microsoft-guidelines.md) for changes, sources, and applicability.

## Why

Out of the box, coding agents write *average* Rust — they clone to dodge the borrow checker, `.unwrap()` everything, and reach for `Box<dyn Trait>` when `impl Trait` would do. These rules encode what expert Rust actually looks like: idiomatic, fast, and safe. Each rule is small and focused, so the agent pulls in only what's relevant to the code in front of it.

## Install

The CLI needs a supported Node.js runtime: `skills` 1.7.6 declares
`node >=22.20.0` (checked 2026-11-10). Use Node.js 24 LTS or 26 LTS; an updated
22 LTS meeting that minimum also works. Check your terminal's version before
running the installer. The skill itself is Markdown and has a
[Git-only installation](#manual-install) that requires no Node.js or npm.
See the [CLI's declared engine](https://github.com/vercel-labs/skills/blob/9442e5adfc63bd0369a4873e53f6b4e6f440bc8f/package.json),
[CLI documentation](https://www.skills.sh/docs/cli), and
[Node.js release status](https://nodejs.org/en/about/previous-releases).

```bash
node --version
npx skills@latest add R5G0/rust-skills
```

The CLI detects your agents and lets you choose the installation destination.

### Installer fails with a missing `styleText` export

[Upstream issue #9](https://github.com/leonardomso/rust-skills/issues/9)
reports this error on Node.js 18.19.1:

```text
SyntaxError: The requested module 'node:util' does not provide an export named 'styleText'
```

Confirm the runtime used by the current terminal:

```bash
node --version
node -p "process.execPath"
node -p "typeof require('node:util').styleText"
```

Node 18 prints `undefined` for the last command. `styleText` was added in Node
20.12.0 and 21.7.0, but those versions do not meet the current CLI's higher
runtime requirement. Install Node 24 LTS or 26 LTS from the
[official Node.js download page](https://nodejs.org/en/download), reopen the
terminal or select that version in your version manager, and repeat the checks.
The selected version must satisfy the CLI's engine and the API check must print
`function`. Then retry:

```bash
npx skills@latest add R5G0/rust-skills
```

The missing export is a Node built-in API; installing an npm package named
`util` does not add it to Node 18. If upgrading Node is unsuitable, follow the
Git-only instructions below and keep the complete skill directory. API history
is documented in [Node.js `util.styleText`](https://nodejs.org/api/util.html#utilstyletextformat-text-options).

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

284 rules split into 26 categories:

| Category | Rules | What it covers |
|----------|-------|----------------|
| **Ownership & Borrowing** | 12 | When to borrow vs clone, Arc/Rc, lifetimes |
| **Error Handling** | 12 | thiserror for libs, anyhow for apps, the `?` operator |
| **Memory** | 17 | SmallVec, arenas, avoiding allocations, `mem::take`, drop order |
| **Unsafe Code** | 9 | `SAFETY:` comments, Miri, `MaybeUninit`, 2024-edition unsafe |
| **API Design** | 22 | Builder pattern, newtypes, sealed traits, `FromIterator` |
| **Async** | 20 | Tokio patterns, channels, async fn in traits, cancel safety |
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
| **Linting** | 15 | Clippy config, CI setup, `unexpected_cfgs` |
| **Anti-patterns** | 16 | Common mistakes and how to fix them |

Each rule has:
- Why it matters
- A bad code example
- A good code example
- Links to related rules and sources

## Validation

CI requires fresh compile evidence for every rule: a complete positive example or
a real Cargo project fixture. The [validation guide](checks/README.md) describes
the checks and their limits; [the report](checks/verification.json) records each
rule's source hash and evidence. Context-dependent supplementary snippets are
listed separately and are not claimed to compile independently. There are no
accepted real type/syntax errors in the suspect baseline.

The [performance project](perf-validation/README.md) also executes correctness
and property tests, Criterion benchmarks and build-profile experiments. Its
[local results](perf-validation/results/summary.md) distinguish observed timings
from general recommendations.

## How it works

The design is built for low token cost and easy auditing:

- **[`SKILL.md`](./SKILL.md)** is a lightweight index — every rule listed as a one-line summary, grouped by category, with a link to its file. The agent reads this first.
- **[`rules/`](./rules)** holds one Markdown file per rule (`<prefix>-<name>.md`). The agent opens only the handful relevant to your code instead of loading the full corpus — progressive disclosure keeps context small.
- **Prefixes** (`own-`, `err-`, `unsafe-`, `async-`, …) map directly to categories, so an agent reviewing async code can pull just `async-`, `conc-`, and `own-` rules.

`CLAUDE.md` and `AGENTS.md` are symlinks to `SKILL.md`, so the same content works across agent conventions.

## Manual install

This route needs Git only; it does not run the Node installer or install global
npm packages. Clone the complete repository so the index can resolve every rule
and reference file. Choose the command for your agent; the destination must be
new or empty.

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

The installed directory must contain all three:

```text
rust-skills/
├── SKILL.md
├── rules/
└── references/
```

For a project installation under `.agents/skills`, record the installed revision
with `git -C .agents/skills/rust-skills rev-parse HEAD`. Restart or reload your
agent's skill discovery after installing.

For Aider, the cloned entrypoint can be supplied with:

```bash
aider --read path/to/rust-skills/SKILL.md
```

## All rules

See [SKILL.md](./SKILL.md) for the full list with links to each rule file.

## Sources & attribution

The inherited rule collection comes from [leonardomso/rust-skills at fd2a861](https://github.com/leonardomso/rust-skills/tree/fd2a861ab0406a4ac536a55274d14ea6fd1ca9c9). Its [README at that revision](https://github.com/leonardomso/rust-skills/blob/fd2a861ab0406a4ac536a55274d14ea6fd1ca9c9/README.md) identifies official Rust documentation, books, and open-source crates as references. These are community-maintained recommendations, not rules issued or endorsed by the Rust project.

This fork adds corrections and source-backed guidance. [The source review](references/source-review.md) records the reviewed revisions, and changed rules link to the documentation supporting their contracts. A reference list is not a claim that every inherited example was independently audited or reproduced.

**Official Rust documentation**
- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe code)
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Standard library docs](https://doc.rust-lang.org/std/) and [release notes](https://doc.rust-lang.org/releases.html)

**Books & guides**
- [Microsoft Pragmatic Rust Guidelines v2026.6](https://microsoft.github.io/rust-guidelines/guidelines/) — adopted rules and conditional policies are listed in [the Microsoft mapping](references/microsoft-guidelines.md)
- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) and [Comprehensive Rust](https://google.github.io/comprehensive-rust/) — selected portable guidance, mapped in [the source review](references/source-review.md)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/) — Nicholas Nethercote
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) — rust-unofficial
- [Rust Atomics and Locks](https://marabos.nl/atomics/) — Mara Bos
- [Effective Rust](https://effective-rust.com/) — David Drysdale

**Tooling**
- [Clippy lint documentation](https://rust-lang.github.io/rust-clippy/)
- [Miri](https://github.com/rust-lang/miri)

**Codebases listed as references by upstream**
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
