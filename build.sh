#!/usr/bin/sh -xe

rustc -C opt-level=3 -C strip=symbols src/example.rs
