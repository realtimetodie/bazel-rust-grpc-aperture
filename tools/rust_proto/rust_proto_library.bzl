"Rust Protocol Buffer library"

load("@crate_index//:defs.bzl", "all_crate_deps")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

def rust_proto_library_macro(name, protos, root, **kwargs):
    """
    A wrapper macro around tonic-build that uses rust_library.

    https://docs.rs/tonic-build/latest/tonic_build/

    Args:
        name: The name of generated library target
        protos: List of proto_library targets containing the .proto sources to generate code from.
        root: The include path to be specified relative to the parent directory
        **kwargs: Other attributes
    """

    proto_types = "%s_proto_types" % name

    native.genrule(
        name = proto_types,
        srcs = ["@com_google_protobuf//:well_known_type_protos"],
        outs = ["%s_proto_types_manifest" % name],
        cmd = "for f in $(rootpaths @com_google_protobuf//:well_known_type_protos); do echo $$f >> $@; done",
    )

    cargo_build_script(
        name = "%s_build_script" % name,
        srcs = ["//tools/rust_proto:build.rs"],
        build_script_env = {
            "PROTOC": "$(execpath @com_google_protobuf//:protoc)",
            "PROTO_ROOT": root,
            "PROTO_TYPES": "$(execpath %s)" % proto_types,
        },
        deps = ["@rules_rust//tools/runfiles"] +
               all_crate_deps(build = True, package_name = "tools/rust_proto"),
        data = protos + [
            proto_types,
            "@com_google_protobuf//:protoc",
            "@com_google_protobuf//:well_known_type_protos",
        ],
    )

    crate_name = kwargs.pop("crate_name", name)
    deps = kwargs.pop("deps", [])

    rust_library(
        name = name,
        crate_name = crate_name,
        deps = [":%s_build_script" % name] + deps,
        **kwargs
    )
