# rust_proto

## Rules

### rust_proto_library

```starlark
load("//tools/rust_proto:index.bzl", "rust_proto_library")
```

```starlark
rust_proto_library(name, protos, root, **kwargs)
```

A wrapper macro around tonic-build that uses rust_library.
