#!/bin/bash

# This is bad, but post build scripts still not implemented yet for cargo
# https://github.com/rust-lang/cargo/issues/545

# 签名配置
codesign -f -s - --timestamp=none --entitlements assets/app.entitlements ./target/*/FurinaOCR*
