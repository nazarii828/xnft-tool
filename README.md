# create-xnft

A simple CLI tool to help you quickly scaffold an xNFT app.

### installation
To install this tool

1. use cargo
```shell
cargo install create-xnft
```

2. Download the binary from the releases page.  

### Quickstart

To quickly scaffold your app

```shell 
create-xnft init 
```
add the `rn` flag to initialize with react-native
```shell
create-xnft init --rn
```


### templates
To use the default [xnft-quickstart](https://github.com/coral-xyz/xnft-quickstart/) template
```shell
create-xnft template --d
```

To list the available templates
```shell
create-xnft template list
```

To use a listed templates
```shell
create-xnft templates get <template-name>
```
