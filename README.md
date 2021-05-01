# Pouf

## Intro

Pouf is a cli program for produce fake datas.

## Install on your system

Latest with source :

clone the projet and install it with :

```bash
cargo install
```

## Examples

```bash
$ pouf internet.mail -l en
>>> adriel_quia@hotmail.com
```

```bash
$ pouf internet.mail // if locales is "fr_FR.UTF-8"
>>> karim_qui@orange.fr
```

```bash
$ pouf finance.bic
>>> RSJECUA1x0hf8NV2FDvN5m8MFV
```

```bash
$ pouf time.time
>>> 21:45:53
```

```bash
$ pouf filesystem.mimetype
>>> application/vnd.xacml+json
```

```bash
$ pouf http.code
>>> 412 Precondition Failed
```

```bash
$ pouf administrative.healthinsurrancecode
>>> 1 85 02 974 777 624 88
```
