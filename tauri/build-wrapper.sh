#!/bin/bash
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"
export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1"
cargo build "$@"
