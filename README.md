# rust-analyzer Regressions - async fn TAIT

rust-analyzer crashes when `TAIT` (trait alias implement trait) is used for async functions. This is not the case for sync functions.

You can observe this yourself, by removing the `async` keyword in the function definition of `foo` in `src/main.rs`.

To reproduce the error:

```bash
$ rustup run nightly-2024-05-08 rust-analyzer --version
rust-analyzer 1.80.0-nightly (faefc618 2024-05-07)

# (async fn)
$ rustup run nightly-2024-05-08 rust-analyzer analysis-stats .
Database loaded:     440.07ms, 0b (metadata 256.38ms, 0b; build 51.32ms, 0b)
  item trees: 1
Item Tree Collection: 1.12ms, 0b
  crates: 1, mods: 1, decls: 3, bodies: 2, adts: 0, consts: 0
Item Collection:     1.81s, 0b
Body lowering:       426.50µs, 0b
0/2 0% processing: regressions::foo
thread 'main' has overflowed its stack
fatal runtime error: stack overflow

# (fn)
$ rustup run nightly-2024-05-08 rust-analyzer analysis-stats .
Database loaded:     342.87ms, 0b (metadata 168.65ms, 0b; build 48.36ms, 0b)
  item trees: 1
Item Tree Collection: 221.67µs, 0b
  crates: 1, mods: 1, decls: 3, bodies: 2, adts: 0, consts: 0
Item Collection:     1.85s, 0b
Body lowering:       280.13µs, 0b
  exprs: 3, ??ty: 0 (0%), ?ty: 0 (0%), !ty: 1
  pats: 0, ??ty: 0 (100%), ?ty: 0 (100%), !ty: 0
Inference:           293.82ms, 0b
MIR lowering:        74.79µs, 0b
Mir failed bodies: 1 (50%)
Data layouts:        167.00ns, 0b
Failed data layouts: 0 (100%)
Const evaluation:    208.00ns, 0b
Failed const evals: 0 (100%)
Total:               2.14s, 0b
```
