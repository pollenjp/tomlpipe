# tomlpipe

## TODO

### `tomlpipe override`

- [x] 標準入力 or `--in <file>` でベースとなる toml を受け取る
- [x] `--override-toml <file>` 上書きする toml を渡す
- [ ] `--override-toml-dot <toml-dot>=<val>` `foo.bar=baz`
- [ ] `--override-toml-dot-type <type>` (default: `String`)
  - `String`
  - `int`
  - `bool`
