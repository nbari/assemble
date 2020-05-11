# assemble
`assemble` following a set of defined instructions in a YAML file `asm.yml`

The `asm.yml` file:

```yaml
---
env:
  KEY: value

steps:
  - name: step 1
    cmd: test

  - name: step 2
    cmd: test

deploy:
  - provider: s3
    access_key_id: key_id
    secret_access_key: access_key
    bucket: "s3 bucket"
    src: app.tar.xz
```
