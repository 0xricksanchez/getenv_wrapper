## README

This is a simple wrapper that hooks calls to `getenv` or `secure_getenv` and replaces the value of the environment variable specified in `argv[1]` with data from stdin.

## Usage

```bash
clang -ggdb -O2 tests/env.c -o tests/env
cargo build
LD_PRELOAD=./target/debug/libets.so ./tests/env ZSH <<< $(head /dev/urandom)
```

Note: The changed environment variable only persist in the context of the run application.
