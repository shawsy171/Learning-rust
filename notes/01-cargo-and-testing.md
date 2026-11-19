# Module 1 · Lesson 1: Cargo and your first tests

## Cargo, mapped to the JS world
| Cargo | Roughly in JS | Notes |
|---|---|---|
| `cargo new foo` / `--lib` | `npm init` | binary crate (`src/main.rs`) or library (`src/lib.rs`) |
| `Cargo.toml` | `package.json` | `[dependencies]`; `Cargo.lock` = lockfile |
| `cargo build` | `tsc` / bundler | output in `target/debug`; `--release` for optimised |
| `cargo run` | `node index.js` | builds then runs the binary |
| `cargo check` | `tsc --noEmit` | type/borrow checks only, no codegen. Fastest feedback loop |
| `cargo test` | `jest` | built in. Unit tests, integration tests and doc tests |
| `cargo fmt` | prettier | one official style, no config debates |
| `cargo clippy` | eslint | idiom + correctness lints. Run it constantly while learning |

Workspace: one root `Cargo.toml` with `[workspace] members`, one shared `target/` and `Cargo.lock`. Like an npm workspace/monorepo.

## Syntax essentials
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b          // no semicolon: this is the return value (blocks are expressions)
}

let x = 5;         // immutable by default
let mut y = 5;     // opt in to mutation
let label = if x > 3 { "big" } else { "small" }; // if is an expression (no ternary)
```
- Types: `i32`, `u64`, `f64`, `bool`, `char` (4-byte Unicode scalar), `&str`.
- Numbers never coerce: `1 + 2.0` is a compile error; use `as f64`.
- `&'static str` = a reference to string data baked into the binary (read-only data segment). String literals are this.

## Unit tests
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]           // only compiled for `cargo test`; zero cost in real builds
mod tests {
    use super::*;      // tests are a child module, so they can see private items too

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```
- `assert!(cond)`, `assert_eq!(left, right)`, `assert_ne!`. All take an optional message: `assert_eq!(a, b, "for input {x}")`.
- The `!` means it's a **macro**, not a function (expands at compile time; that's how it prints both values on failure).
- A test passes if it doesn't panic.
- Floats: never `assert_eq!` computed floats. Compare with a tolerance: `assert!((a - b).abs() < 1e-9)`.

## Useful test commands
- `cargo test -p m01_basics` — one crate
- `cargo test describe` — only tests whose name contains `describe`
- `cargo test -- --nocapture` — show `println!` output
