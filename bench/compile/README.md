# Compile benchmarks

These benchmarks are designed to measure the cost of consuming async-stripe (and related crates) as a library,
specifically
around compilation time and binary size. Benchmarks are simple rust binaries consuming the library in different ways.
They are just meant
to be compiled, never run.

The collector builds each selected consumer in an isolated temporary target directory,
reads [Cargo's artifact messages](https://doc.rust-lang.org/1.88.0/cargo/reference/external-tools.html#json-messages)
through `cargo_metadata`, and writes readable `summary.md` and
`results.json` files.

The input is the library source already present in each checkout, including its
committed generated Rust. Comparing revisions requires collecting each checkout
and comparing their reports.

```sh
# Default: minimal + requests, one clean build each in dev and release.
cargo run --quiet --locked -p compile-bench -- collect

# All five workloads; keep this directory as a baseline.
cargo run --quiet --locked -p compile-bench -- collect \
  --cases all --output target/compile-benchmarks/baseline

# After a library change, use the same toolchain and build settings.
cargo run --quiet --locked -p compile-bench -- collect \
  --cases all --output target/compile-benchmarks/current
cargo run --quiet --locked -p compile-bench -- compare \
  target/compile-benchmarks/baseline target/compile-benchmarks/current
```

Without `--output`, each collection creates a fresh
`target/compile-benchmarks/run-<random>/` directory in the measured checkout and
prints its path. Reports and logs are kept there after the run. Use `--output`
to choose a named directory, as in the comparison example; that directory must
not already exist. Relative `--output` paths resolve against the measured
checkout. Existing results are never overwritten. Reports are replaced
after each build and diagnostic, so completed measurements survive a later
failure. A failed requested build or LLVM diagnostic produces a nonzero exit
status, with details in the report and retained logs.

The collector is an unpublished workspace member outside `default-members`. It
has no Stripe dependencies; its compilation and dependencies are outside the
measured target directories. `cargo run -p compile-bench` compiles the collector
first.

## Workloads

| Name         | Coverage                                                                                                            |
|--------------|---------------------------------------------------------------------------------------------------------------------|
| `minimal`    | Deserialize a customer through the shared types crate.                                                              |
| `poly`       | Deserialize several polymorphic shared types, including the raw event envelope.                                     |
| `deser-many` | Deserialize 31 response types through ten public domain crates, without unused request features.                    |
| `requests`   | Await representative retrieves, creates, and pagination through the async client.                                   |
| `webhook`    | Verify signatures and dispatch typed webhook events, with the webhook crate's `full` feature (heavily polymorphic). |

## Default measurements

Each case/profile costs one build. From that build the collector records:

- Original and stripped executable bytes.
- Every executable section's name, segment, file and memory size (read by the
  `object` crate), with text and debug totals in the main table. Text totals use
  `object`'s `SectionKind::Text`; other executable sections, such as Mach-O stubs,
  may have different kinds and are excluded from that column. Debug totals
  include ELF `.debug_*`/`.zdebug_*` and Mach-O DWARF sections; compressed sections
  count their physical bytes. Other section categories remain raw JSON data,
  since their classification varies by object format.
- Per-library `.rlib` and `.rmeta` sizes for first-party libraries, with their
  enabled features. These come only from artifacts reported by this build.
- Dev incremental-cache bytes, split between the consumer and Stripe libraries.
  These are collected from the same initial clean build, before temporary target
  directories are deleted. There are no extra builds or rebuild timings.
- Compiler, Cargo, target, build settings, commit/dirty state, lockfile hash and
  workload hashes, to help assess comparisons. Workspace profile inputs include
  package and build overrides; Cargo's artifact profile fields are also retained.

`summary.md` keeps run metadata, measurement tables and any warnings/errors,
with per-library breakdowns in collapsible tables. `results.json` keeps exact
bytes, every section and recorded build settings.

Section file sizes exclude zero-fill memory, headers and symbol tables. The
Stripe library columns total first-party build artifacts, not download or
linked-code sizes. `rlib` archives already contain
metadata, so `rlib` and separate `rmeta` totals describe different on-disk
artifacts rather than additive code sizes. Object sections and stripping are
best effort: unavailable values are explicit, with warnings. Native macOS and
Linux are the intended environments; a cross target may need a matching linker
and strip tool.

Both `dev` and `release` are measured by default. The collector explicitly pins
`RUSTC` for measured builds to the compiler whose version it records: `RUSTC`,
then `CARGO_BUILD_RUSTC`, then `rustc` from PATH (in the measured checkout, so
rustup honors its toolchain). A Cargo configuration file's `build.rustc` setting
is overridden; use either environment variable to select a custom compiler.
Incremental compilation is enabled for dev and disabled for release;
compiler wrappers are disabled. Dev artifact sizes therefore include the effects
of incremental compilation, as in a normal dev build.
As [documented by rustc 1.88](https://doc.rust-lang.org/1.88.0/rustc/codegen-options/index.html#split-debuginfo),
`split-debuginfo=off` keeps DWARF in ELF
executables; on macOS it skips dSYM generation and leaves DWARF in object
files/rlibs. Thus zero executable debug-section bytes on macOS does not mean zero
generated debug data; library artifact sizes still reflect that storage. Profile
stripping is disabled before measuring the original. Other profile settings come
from Cargo as usual. Source paths under the repository, temporary target and Cargo home are remapped to stable
prefixes, reducing debug-path differences across checkouts. The collector appends
these remaps to ambient `RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS`; as with Cargo's global
flags, these take precedence over flags in Cargo configuration. Avoid relying on
target-specific configured rustflags for these measurements.

Incremental sizes sum regular-file lengths, not allocated disk blocks. Cache
directories are matched to Cargo artifact target names using rustc's
`<crate_name>-<id>` naming convention; cache contents are not parsed. A missing
expected cache is unavailable with a warning. The consumer total describes the
benchmark binary; the library total covers only Stripe library targets reported
by the build. Since Cargo uses incremental compilation for workspace/path
dependencies, the library caches reflect developing against this checkout.
Registry dependencies do not contribute incremental caches. These measurements
describe the initial build, not cache growth across edits.

`compare` accepts report directories or `results.json` files. Comparisons match
workload and profile, show exact byte and percentage deltas,
and identify added or removed rows. Different
toolchains, platforms, targets, recorded environment settings, Cargo configuration
hashes, workspace profile tables or Cargo artifact profile fields are rejected unless
`--allow-mismatch` is explicit; failures name the differing environment fields.
Changed workload hashes, incomplete reports and measurement warnings are called
out. Binary sizes can still change with compiler/linker layout; exact reproducibility is not a
promise.

## Optional diagnostics and progress

```sh
# Two clean release builds per selected workload, with wall time and rough RSS.
cargo run --quiet --locked -p compile-bench -- collect \
  --profiles release --timings 2 --output target/compile-benchmarks/timed

# Additional IR compilation; requires cargo-llvm-lines (tested with 0.4.46).
cargo run --quiet --locked -p compile-bench -- collect \
  --cases minimal --profiles release --llvm-lines \
  --output target/compile-benchmarks/ir
```

`--timings N` uses N total clean builds, including the first size build; it does
not add N builds on top. Downloads are fetched before timing samples, then
builds run offline. Cargo fetch is workspace-wide and can download dependencies
unused by the selected fixtures. `--offline` skips fetching and forbids network
access. Wall times include Cargo and log handling; filesystem caches are not
cleared. Timing and RSS are noisy diagnostics.
Raw samples are retained. Optional RSS uses `/usr/bin/time` on macOS or GNU `time`
on Linux; it reports a rough largest-process peak, not simultaneous process-tree
memory. If unavailable, wall-time collection continues.

`--llvm-lines` records the selected consumer root and, once per profile, a
standalone default-feature `async-stripe-shared` library build. Neither count
includes separately compiled dependencies, and they must not be added to claim a
whole-program total. Full IR reports remain under `logs/`. A missing LLVM tool
is reported as an error while ordinary size collection continues.
