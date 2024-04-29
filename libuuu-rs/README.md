# `libuuu-rs`

This package exposes rust bindings to `libuuu` for integrating flashing for I.MX devices using a custom application.

## Linux

Tested on `Ubuntu 22.04`

### Intel, AMD (`amd64`)
```bash
$ export LIBUUU_RS_CLANG_ARGS="-I../libuuu -I/usr/include -I/usr/lib/gcc/x86_64-linux-gnu/11/include -I/usr/include/libusb-1.0"
$ cargo build --release
```

### ARM64
```bash
$ export LIBUUU_RS_CLANG_ARGS=
$ cargo build --release
```

## MacOS

### AMD64

```bash
TODO
```

### ARM64

```bash
TODO
```