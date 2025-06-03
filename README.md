# handler_table

[![Crates.io](https://img.shields.io/crates/v/handler_table)](https://crates.io/crates/handler_table)
[![Docs.rs](https://docs.rs/handler_table/badge.svg)](https://docs.rs/handler_table)
[![CI](https://github.com/arceos-org/handler_table/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/arceos-org/handler_table/actions/workflows/ci.yml)

A lock-free, `no_std`, fixed-capacity table of event handlers.

## Features

- no_std, no heap allocation
- Lock-free registration, unregistration and invocation
- Generic capacity `N` known at compile time
- Suitable for bare-metal and OS-kernel contexts

## Examples

```rust
use handler_table::HandlerTable;

static TABLE: HandlerTable<8> = HandlerTable::new();

assert!(TABLE.register_handler(0, || {
   println!("Hello, event 0!");
}));
assert!(TABLE.register_handler(1, || {
   println!("Hello, event 1!");
}));

assert!(TABLE.handle(0)); // print "Hello, event 0!"
assert!(!TABLE.handle(2)); // unregistered

assert!(TABLE.unregister_handler(2).is_none());
let func = TABLE.unregister_handler(1).unwrap(); // retrieve the handler
func(); // print "Hello, event 1!"

assert!(!TABLE.handle(1)); // unregistered
```
