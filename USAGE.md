> sadly, uploading this to crates.io would cause heavy namespace conflicts...
> (it's a workspace)

simply clone the repo and refer to it in your Cargo.toml like this:

```toml

[dependencies]
age_engine={ path="path/to/age_engine" }
```
