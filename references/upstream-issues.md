# Upstream issue coverage

Checked on 2026-11-10 against GitHub's complete issues API, including open and
closed results and excluding pull requests. It returned six actual issues:
#1, #3, #6, #8, #9 and #10. All six have a resolution in this fork. Their upstream
GitHub state is separate from this fork's implementation status.

| Issue | Resolution in this fork | Maintained evidence |
|---|---|---|
| [#1: source of rules](https://github.com/leonardomso/rust-skills/issues/1) | README identifies the inherited upstream commit, upstream's declared bibliography, and this fork's source-backed additions without claiming an unaudited authorship history | [README attribution](../README.md#sources--attribution), [source provenance](source-review.md#provenance) |
| [#3: Microsoft guidelines](https://github.com/leonardomso/rust-skills/issues/3) | Applicable guidance is integrated into rules; every published guideline at the pinned source revision has an explicit mapping or applicability decision | [Microsoft mapping](microsoft-guidelines.md), [skill index](../SKILL.md) |
| [#6: async trait dyn compatibility](https://github.com/leonardomso/rust-skills/issues/6) | Distinguish Send from dyn compatibility; use async-trait or explicit boxed futures for dynamic dispatch | [async-fn-in-trait](../rules/async-fn-in-trait.md) |
| [#8: ownership correctness and clarity](https://github.com/leonardomso/rust-skills/issues/8) | Correct all three concrete defects and all seven ownership clarifications | The ten rule mappings below |
| [#9: Ubuntu installation on Node 18](https://github.com/leonardomso/rust-skills/issues/9) | State the current CLI's actual runtime requirement, diagnose missing styleText, and provide a complete Git-only installation requiring no Node upgrade | [installer troubleshooting](../README.md#installer-fails-with-a-missing-styletext-export), [manual install](../README.md#manual-install) |
| [#10: unnecessary clone and false Rc-to-Send claim](https://github.com/leonardomso/rust-skills/issues/10) | Move an existing Arc, borrow when needed, and copy deliberately for a snapshot; explain why ordinary borrows can remain Send and cloning Rc cannot make it Send | [async-clone-before-await](../rules/async-clone-before-await.md), [lock consistency](../rules/async-no-lock-await.md) |

## Issue #8: all ten reviewed rules

| Requirement | Rule |
|---|---|
| Path accepts Path/PathBuf references, not implicit string conversion | [own-slice-over-vec](../rules/own-slice-over-vec.md) |
| Mutex size is target-dependent; parking_lot has different poisoning and return contracts | [own-mutex-interior](../rules/own-mutex-interior.md) |
| Stable downgrade changes lock mode, not scheduling fairness | [own-rwlock-readers](../rules/own-rwlock-readers.md) |
| Clone is not universally a heap allocation or pointee copy | [own-borrow-over-clone](../rules/own-borrow-over-clone.md) |
| Distinguish Cow into_owned from to_mut and their copy/allocation paths | [own-cow-conditional](../rules/own-cow-conditional.md) |
| Arc does not bypass its pointee's Send/Sync requirements | [own-arc-shared](../rules/own-arc-shared.md) |
| Choose Rc for a real shared lifetime requirement | [own-rc-single-thread](../rules/own-rc-single-thread.md) |
| Cell avoids dynamic borrow conflicts, not arithmetic overflow | [own-refcell-interior](../rules/own-refcell-interior.md) |
| Copy size bands are heuristics, not language restrictions or free work | [own-copy-small](../rules/own-copy-small.md) |
| Moving an owned Vec is valid; clone when another retained owner needs it | [own-clone-explicit](../rules/own-clone-explicit.md) |

## Verification

An independent Rust 1.97.1 pass compiled and executed four focused positive
programs and confirmed five expected compiler rejections. It covered:

- Dynamic dispatch with async-trait and explicit boxed futures.
- Rejection of dyn dispatch for a trait returning an opaque Send future.
- Rejection of direct str/String-reference calls to a Path-reference parameter.
- Rejection of a cloned Rc future and Arc<RefCell<_>> under Send requirements.
- Path/AsRef conversions, Cow transitions, large Copy arrays and valid Vec moves.
- Counted element clones: keeping an Arc handle or reborrowing copied no
  elements; a deep Vec copy copied each element.
- Atomic write-to-read downgrade and lock/poisoning contracts.

The standard extraction harness separately checks the rule examples against a
reviewed inherited compile-suspect baseline. That baseline is not the issue
list and is not evidence that every illustrative fragment compiles standalone.
See [the compile-check documentation](https://github.com/R5G0/rust-skills/blob/master/checks/README.md) for the exact scope and command.
