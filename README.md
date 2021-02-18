# dragon_dmc

dragon_dmc is an implementation of the DMCv2 protocol

## C Bindings

To generate the C header file, you must have cbindgen tool installed

```sh
cargo install --force cbindgen
```

Then you can generate the header with

```sh
cbindgen --config cbindgen.toml --crate dragon_dmc --output dragon_dmc.h
```
