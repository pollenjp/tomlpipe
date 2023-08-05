# tomlpipe

A CLI tool to edit TOML config file.

## Install

```sh
cargo install tomlpipe
```

## TODO

### `tomlpipe override`

- [x] 標準入力 or `--in <file>` でベースとなる toml を受け取る
- [x] `--override-toml <file>` 上書きする toml を渡す
- [ ] `--override-toml-dot <toml-dot>=<val>` `foo.bar=baz`
- [ ] `--override-toml-dot-type <type>` (default: `String`)
  - `String`
  - `int`
  - `bool`

### How to use

```sh
tomlpipe override --override-toml sample_override.toml < sample_base.toml
```

`sample_base.toml`

```toml
# aaa
aaa = "bbb"

[a.b.c]
d = true
```

`sample_override.toml`

```toml
[a.b.c]
d = false
```

output

```toml
# aaa
aaa = "bbb"

[a.b.c]
d = false
```
