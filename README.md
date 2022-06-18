# Pouf

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/pouf.svg)](https://crates.io/crates/pouf)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.60.0+-lightgray.svg)](#rust-version-requirements)

## Intro

Pouf is a cli program to produce fake datas.

## Last stable version

[![Packaging status](https://repology.org/badge/vertical-allrepos/pouf.svg)](https://repology.org/project/pouf/versions)

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

### Address

```zsh
$ pouf address.country --lang en
Congo
```

```zsh
$ pouf address.city --lang en
Carter burgh
```

```zsh
$ pouf address.country --lang en
Wunsch Vista
```

### Administrative

```zsh
$ pouf administrative.healthinsurrancecode --lang fr
1 85 02 974 777 624 88
```

### Http

```zsh
$ pouf http.code
412 Precondition Failed
```

### Internet

```zsh
$ pouf internet.mail -l en
adriel_quia@hotmail.com
```

```zsh
$ pouf internet.mail // if locales is "fr_FR.UTF-8"
karim_qui@orange.fr
```

### Filesystem

```zsh
$ pouf filesystem.mimetype
application/vnd.xacml+json
```

```zsh
$ pouf filesystem.semver
filesystem.semver
```

### Finance

```zsh
$ pouf finance.bic
RSJECUA1x0hf8NV2FDvN5m8MFV
```

### People

```zsh
$ pouf people.name
Axel Sipes
```

### Time

```zsh
$ pouf time.time
21:45:53
```

### multiple launch

You can launch n time like this :

```zsh
$ pouf finance.bic -n 5
TLNINTG1361
HJGOCSK1
VUKIBZB1
RJCIAZV1177
TVGOSIC1
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
