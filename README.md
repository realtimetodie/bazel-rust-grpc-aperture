# bazel-rust-grpc-aperture

An example how to use [Tonic](https://github.com/hyperium/tonic) with Bazel

![Portal](/demo.png "game poster")

## Getting started

Hello, and again, welcome to the Aperture Science Enrichment Center.

This example works best in scenarios where the Protocol Buffer .proto definition files are grouped in a single root directory.

Given the following workspace structure

```
[workspace]/
    WORKSPACE
    BUILD
    proto/
        aperture/
            experiments.proto
        src
            lib.rs
```

The following Protocol Buffer namespace in proto/aperture/experiments.proto

```proto
syntax = "proto3";

package aperture.experiments;
        ^^^^^^^^
        Protocol Buffer namespace
```

When a Protocol Buffer .proto definition file changes, the Rust library in `proto/aperture/src/lib.rs` might have to be updated. This part can not be automated.

## Building the source code

```sh
$ bazel build //...
```
