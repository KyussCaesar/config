goals:

- compile-time checked usage of configuration keys
- load configuration at startup, handle errors there?
- can ask the result to explain where it got a value from and why

# Contributing

Clone the repo.

Need to have Docker installed/running.

`make test` to run tests.

# Usage

1. Describe your configuration:

```rust
struct Configuration {
  spline_reticulation_algortithm: String,
}
```

2. Describe configuration sources:

```rust
```

