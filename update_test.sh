#!/bin/sh
cargo update && \
./test.sh && \
./manual_test.sh
