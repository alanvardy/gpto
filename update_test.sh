#!/bin/bash

cargo update && \
./test.sh && \
./manual_test.sh
