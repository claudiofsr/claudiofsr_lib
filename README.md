# claudiofsr_lib
General-purpose library used by my programs.

## Building

To use this library, add in `Cargo.toml`:
```
[dependencies.claudiofsr_lib]
version = "0.19"
# git = "https://github.com/claudiofsr/claudiofsr_lib"

```

To use this library as a dependency with the decimal feature enabled,
add the following to your project's Cargo.toml:

Cargo.toml:
```
[dependencies]
claudiofsr_lib = { version = "0.19", features = ["decimal"] }
```

See the [documentation](https://docs.rs/claudiofsr_lib/latest/claudiofsr_lib/).
