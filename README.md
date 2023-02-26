# Pre-requisites
If running from source, Rust programming language needs to be installed.

# Usage
## From source
```sh
steam-achievement-unlocker.exe <AppId> <achievement name>
```
## From binary
```sh
steam-achievement-unlocker.exe <AppId> <achievement name>
```

# Examples
## From source
```sh
# Unlock achievement "PURITY" from AppId 646570 (slay the spire)
cargo run -- 646570  "PURITY"
```

## From binary
```sh
# Unlock achievement "PURITY" from AppId 646570 (slay the spire)
steam-achievement-unlocker.exe 646570  "PURITY"
```