# Pouf

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/pouf.svg)](https://crates.io/crates/pouf)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.60.0+-lightgray.svg)](#rust-version-requirements)

## Intro

Pouf is a cli program for produce fake datas.

## Install with crates.io

```zsh
cargo install pouf
```

## Install on your system

Latest with source :

clone the projet and install it with :

```zsh
cargo install --path .
```

## Examples

```zsh
$ pouf internet.mail -l en
adriel_quia@hotmail.com
```

```zsh
$ pouf internet.mail // if locales is "fr_FR.UTF-8"
karim_qui@orange.fr
```

```zsh
$ pouf internet.color
#AAAA9A
```

```zsh
$ pouf finance.bic
RSJECUA1x0hf8NV2FDvN5m8MFV
```

```zsh
$ pouf time.time
21:45:53
```

```zsh
$ pouf filesystem.mimetype
application/vnd.xacml+json
```

```zsh
$ pouf filesystem.semver
filesystem.semver
```

```zsh
$ pouf http.code
412 Precondition Failed
```

```zsh
$ pouf administrative.healthinsurrancecode
1 85 02 974 777 624 88
```

## Autocomplete

On zsh :

Add this on your ~/.zshrc :

```zsh
fpath=("dir_of/_pouf" "${fpath[@]}")
```

before :
```zsh
autoload -Uz compinit && compinit
```

## Publish

```zsh
cargo publish --no-verify
```
