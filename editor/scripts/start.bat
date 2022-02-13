@echo off
echo "open http://0.0.0.0:8080"
(cd docs && basic-http-server --addr 0.0.0.0:8080 .)
