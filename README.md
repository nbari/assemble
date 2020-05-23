# assemble

[![crates.io](https://img.shields.io/crates/v/assemble.svg)](https://crates.io/crates/assemble)
[![Build Status](https://travis-ci.org/nbari/assemble.svg?branch=master)](https://travis-ci.org/nbari/assemble)

`build` & `deploy` following a set of defined instructions in a YAML file `asm.yml`

The `asm.yml` file:

```yaml
---
name: <name of the deployment>
version: <commit or date in ISO 8601>

env: # key-value environment variables
  KEY: <value>

build: # list of steps to do in order
  - <shell command to run>
  - name: <name of the step>
    do: <shell command to run>
    get: <s3/name/commit/item>
    put: <s3/name/commit/item>

deploy: # steps for deploying
  - name: <name of the step>
    do: <shell command to run>
    get: <s3/name/version/item>
    put: <s3/name/version/item>

# S3
storage:
  endpoint: <s3 endpoint>
  region: <s3 region>
  access_key: <s3 access_key>
  secret_key: <s3 secret_key>
  bucket: <s3 bucket>
```
