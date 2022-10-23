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

### Address (English Only)

```zsh
$ pouf address.city
Carter burgh
```

```zsh
$ pouf address.country
Congo
```

```zsh
$ pouf address.street
Tillman Freeway
```

### Administrative (French Only)

```zsh
$ pouf administrative.healthinsurrancecode
1 85 02 974 777 624 88
```

### Licenseplate (French Only)

```zsh
$ pouf auto.licenseplate
QV-951-KA
```

### Color

```zsh
$ pouf color
#434733
rgb(67, 71, 51)
rgba(67, 71, 51, 0.4)
hsl(71, 16%, 24%)
hsl(71, 16%, 24%, 0.4)
```

### Filesystem

```zsh
$ pouf filesystem.mimetype
application/vnd.lotus-1-2-3
```

```zsh
$ pouf filesystem.semver
0.15.0
$ pouf filesystem.semver --stable
4.4.12
$ pouf filesystem.semver --unstable
2.16.8-rc.7
```

### Finance

```zsh
$ pouf finance.bic
YMEEIOX1284
```

### Http

```zsh
$ pouf http.code
412 Precondition Failed
```

### Internet

```zsh
$ pouf internet.ip
196.124.139.106
$ pouf internet.ip --ipv6 true
DFC4:E3DD:6124:DD1:1D69:F2C7:B968:59BD
$ pouf internet.ip --ipv4 true
19.59.17.64
```

```zsh
$ pouf internet.mac
7C:41:B6:CC:A2:67
```

```zsh
$ pouf internet.mail -l en
adriel_quia@hotmail.com
```

```zsh
$ pouf internet.mail // if locales is "fr_FR.UTF-8"
karim_qui@orange.fr
```

### People

```zsh
$ pouf people.name
Axel Sipes
$ pouf people.name -l fr
Gerard Sablonnière
```

### Time

```zsh
$ pouf time.time
21:45:53
```

```zsh
$ pouf time.date
2176-01-27T01:25:42.642830566+00:00
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

## Dev

Makefile inspire by https://git.sr.ht/~julienxx/castor/tree/master/item/Makefile

## Publish

```zsh
make cargo-publish
```
