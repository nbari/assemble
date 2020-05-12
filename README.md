# assemble

`build` & `deploy` following a set of defined instructions in a YAML file `asm.yml`

The `asm.yml` file:

```yaml
---
name: <name of the deployment>
version: <version to deploy>

env: # key-value environment variables
  KEY: <value>

build: # list of steps to do in order
  - name: <name of the step>
    do: <shell command to run>
    get: <get item from storage>
    put: <put item in storage>

deploy: # steps for deploying
  - name: <name of the step>
    do: <shell command to run>
    get: <get item from storage>
    put: <put item in storage>

# list of supported storage types
# needs to support get/put
storage:
  - type: <type like s3>
    key: <value>
```
