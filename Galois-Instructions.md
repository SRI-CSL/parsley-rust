# Introduction

This tool is a static analyzer for the ICC max format. We extract the calculator
elements and the associated functions and subelements using our Parsley Rust
combinators and then build a static analyzer to ensure safe stack and input/output
channel operations.

# Running

The `iccmax` branch would hold the ICC max codebase. Once you pull this branch,
you can simply run the following commands to run the static analyzer.

- `make` builds various binaries in the repo (I use the following compiler: rustc 1.55.0-nightly)
- `./target/debug/iccmax_parser` is the path to final Rust binary. It takes a command line argument that is an ICC profile

# Output

Our parser returns `Ok("")` if the static check succeeds. And returns Err("") if the static check failed at a step.

```
"tests/test_files/icc/v5/icc/Display/LCDDisplay.icc"
Ok("")
"tests/test_files/icc/v5/icc/Display/Rec2100HlgFull.icc"
Ok("")
"tests/test_files/icc/v5/icc/Display/sRGB_D65_MAT-300lx.icc"
Ok("")
"tests/test_files/icc/v5/icc/Display/sRGB_D65_MAT-500lx.icc"
Ok("")
"tests/test_files/icc/v5/icc/Display/GrayGSDF.icc"
Err("Not enough subelements in MPET")
```

We are working on adding some more debugging information to the error messages, but the overall output format should remain stable.
