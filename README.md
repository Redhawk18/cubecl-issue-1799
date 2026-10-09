# Nested `#[cube] fn` + indexing

Minimal reproduction of a cubecl 0.10.0 compile error: a `#[cube]` function defined
*inside* another `#[cube]` function cannot use `[]` indexing.

```
error[E0599]: no method named `__expand_cube_idx_method` found for struct `NativeExpand<T>` in the current scope
  --> src/lib.rs:15:5
   |
15 |     #[cube]
   |     ^^^^^^^
```

## Run it

```sh
cargo build
```

`src/lib.rs` has three functions, and only `nested_index` fails:

- `works`: helper defined at module level that indexes, compiles
- `nested_index`: nested helper that indexes, fails
- `nested_no_index`: nested helper with only arithmetic, compiles

## What happens

The outer `#[cube]` rewrites the entire function body, and a nested `fn` item is part of
that body. The outer pass turns `array[i]` in the inner function into a method call,
`*array.cube_idx(i)`. Then the inner `#[cube]` runs, but by now it sees a method call
instead of an index expression. It expands it the way it expands any method call, into
`array.__expand_cube_idx_method(scope, ...)`, and that method doesn't exist.

The expansion of `sample`, from `cargo rustc --lib -- -Zunpretty=expanded`:

```rust
// nested: inner #[cube] received already-rewritten code
fn sample<T: Float + CubeElement>(array: &Array<T>, i: usize) -> T {
    use cubecl::prelude::{CubeIndex as _, CubeIndexMut as _, ComptimeIndex as _, ComptimeIndexMut as _};
    use cubecl::prelude::{CubeIndex as _, CubeIndexMut as _, ComptimeIndex as _, ComptimeIndexMut as _};
    *array.cube_idx(i)
}
// ...its expand fn:
array.__expand_cube_idx_method(scope, _arg_0.into())   // <- does not exist
```

The duplicated `use` block shows the body went through the macro twice. Compare the same
helper defined at the top level, where `array[i]` is still an index expression when the
macro sees it:

```rust
_array.expand_index(scope, _index.into())              // <- correct
```

A nested helper that only does arithmetic (`nested_no_index`) compiles, so the failure
comes specifically from rewriting the index operator twice.

## Fix

Define `#[cube]` helpers at module level, not inside another `#[cube]` function.
