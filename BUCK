# A list of available rules and their signatures can be found here: https://buck2.build/docs/api/rules/

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

alias(
    name = "calc",
    actual = "//crates/bin/calc:calc",
)

alias(
    name = "syntaxgen",
    actual = "//crates/lib/devtools/syntaxgen:syntaxgen",
)

# alias(
#     name = "mini-inkwell",
#     actual = "//crates/bin/mini-inkwell:mini-inkwell",
# )

prebuilt_rust_library(
    name = "inkwell_build",
    rlib = "target/release/libinkwell_build.rlib",
    visibility = ["PUBLIC"],
)