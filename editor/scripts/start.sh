#!/bin/bash
set -eu
echo "http://0.0.0.0:8080"
(cd ../docs && basic-http-server --addr 127.0.0.1:8080 .)
