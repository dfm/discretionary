# Discretionary

A tiny procedural macro to make all your struct fields optional. For now, this
package is essentially trivial, although it might be useful if you have some
huge `struct` that needs to have all its field types converted from `T` to
`Option<T>`. (I needed that and now this exists!)

## Usage

Add `discretionary` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
discretionary = "0.1"
```

Then decorate your `struct` with the `#[make_optional]` macro. That's it!

For example, the following

```rust
use discretionary::make_optional;

#[make_optional]
struct ExampleStruct {
    id: usize,
    name: String,
}
```

will be re-written to something like

```rust
struct ExampleStruct {
    id: Option<usize>,
    name: Option<String>,
}
```
