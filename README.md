# fix-typelength-limit

This is a small tool that automates the process of fixing the following error
when building Rust programs:

```
error: reached the type-length limit while instantiating `SOME TYPE HERE`
...
note: consider adding a `#![type_length_limit="LIMIT_HERE"]` attribute to your crate
```

It looks for this error and extracts the suggested type length limit from the
error message and updates `src/lib.rs` or `src/main.rs` with that value. In order
for this to work you must have already annotated your source file with a limit
value. The program keeps retrying the build till the build succeeds or fails due
to some other error type.

## Install

Clone the source and run the following to install the program:

```
cargo install --path . --force
```

## Run

Run the program like so from the root of your crate's source:

```
fix-typelength-limit cargo build --release
```

You can add any build option as your normally would.