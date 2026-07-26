#!/usr/bin/env bash

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
runs=${1:-2}
report=${2:-target/compile-benchmark-results.md}
slice_list=${3:-minimal,requests}

# Disable incremental compilation and compiler-wrapper hooks so tools
# like sccache can't reuse compiler output
export CARGO_INCREMENTAL=0
export RUSTC_WRAPPER=
export RUSTC_WORKSPACE_WRAPPER=

if [[ ! "$runs" =~ ^[1-9][0-9]*$ ]]; then
  echo "usage: $0 [runs] [report-path] [comma-separated-slices|all]" >&2
  exit 2
fi

if [[ "$slice_list" == all ]]; then
  slice_list=minimal,poly,deser-many,requests
fi

IFS=, read -r -a slices <<< "$slice_list"
for slice in "${slices[@]}"; do
  case "$slice" in
    minimal | poly | deser-many | requests) ;;
    *)
      echo "unknown compile benchmark slice: $slice" >&2
      exit 2
      ;;
  esac
done

if ! command -v hyperfine >/dev/null; then
  echo "hyperfine is required (for example: brew install hyperfine)" >&2
  exit 1
fi
installed_hyperfine=$(hyperfine --version)

case "$(uname -s)" in
  Darwin)
    time_command="/usr/bin/time -l"
    rss_unit=bytes
    ;;
  Linux)
    time_command="/usr/bin/time -f %M"
    rss_unit=kib
    ;;
  *)
    time_command=
    rss_unit=
    echo "maximum RSS measurement is unavailable on this platform; continuing without it" >&2
    ;;
esac

if ! installed_llvm_lines=$(cargo llvm-lines --version 2>/dev/null); then
  echo "cargo-llvm-lines is required" >&2
  echo "install it with: cargo install --locked cargo-llvm-lines" >&2
  exit 1
fi

if [[ "$report" != /* ]]; then
  report="$repo_root/$report"
fi

mkdir -p "$(dirname -- "$report")"

compile_bench_target_root=$(mktemp -d "${TMPDIR:-/tmp}/async-stripe-compile-benchmarks.XXXXXX")
export COMPILE_BENCH_TARGET_ROOT="$compile_bench_target_root"
trap 'rm -rf "$compile_bench_target_root"' EXIT

cd "$repo_root"
cargo fetch --locked

timings="$compile_bench_target_root/timings.md"
build_command="env CARGO_TARGET_DIR=\"\$COMPILE_BENCH_TARGET_ROOT/{slice}\" cargo build --release --locked --offline -p \"bench-{slice}\""
if [[ -n "$time_command" ]]; then
  build_command="$time_command -a -o \"\$COMPILE_BENCH_TARGET_ROOT/{slice}.max-rss\" $build_command"
fi
hyperfine \
  --runs "$runs" \
  --parameter-list slice "$slice_list" \
  --prepare 'rm -rf "$COMPILE_BENCH_TARGET_ROOT/{slice}"' \
  --command-name 'bench-{slice} (clean release)' \
  --export-markdown "$timings" \
  "$build_command"

# llvm-lines recompiles only the selected root target to emit unoptimized
# textual IR. Dependencies remain reusable, but their separately emitted IR
# is not included.
llvm_lines_dir="$compile_bench_target_root/llvm-lines"
mkdir -p "$llvm_lines_dir"

for slice in "${slices[@]}"; do
  target_dir="$compile_bench_target_root/$slice"
  binary="$target_dir/release/bench-$slice"
  wc -c < "$binary" | tr -d '[:space:]' > "$compile_bench_target_root/$slice.unstripped-bytes"

  cargo llvm-lines \
    --release \
    --locked \
    --offline \
    --target-dir "$target_dir" \
    -p "bench-$slice" \
    --bin "bench-$slice" \
    > "$llvm_lines_dir/bench-$slice.txt"
done

cargo llvm-lines \
  --release \
  --locked \
  --offline \
  --target-dir "$compile_bench_target_root/${slices[0]}" \
  -p async-stripe-shared \
  --lib \
  > "$llvm_lines_dir/async-stripe-shared.txt"

for slice in "${slices[@]}"; do
  binary="$compile_bench_target_root/$slice/release/bench-$slice"
  strip "$binary"
  wc -c < "$binary" | tr -d '[:space:]' > "$compile_bench_target_root/$slice.stripped-bytes"
done

if [[ -n "${GITHUB_SHA:-}" ]]; then
  commit=$GITHUB_SHA
else
  commit=$(git rev-parse HEAD)
  if [[ -n "$(git status --porcelain)" ]]; then
    commit="$commit-dirty"
  fi
fi

runner=${RUNNER_OS:-$(uname -s)}/${RUNNER_ARCH:-$(uname -m)}

llvm_totals() {
  awk '$3 == "(TOTAL)" { print $1, $2; found = 1; exit } END { if (!found) exit 1 }' "$1"
}

max_rss_mib() {
  if [[ "$rss_unit" == bytes ]]; then
    awk '
      $2 == "maximum" && $3 == "resident" && $4 == "set" && $5 == "size" {
        if (!found || $1 > max) max = $1
        found = 1
      }
      END { if (!found) exit 1; printf "%.2f", max / 1048576 }
    ' "$1"
  else
    awk '
      NF == 1 && $1 ~ /^[0-9]+$/ {
        if (!found || $1 > max) max = $1
        found = 1
      }
      END { if (!found) exit 1; printf "%.2f", max / 1024 }
    ' "$1"
  fi
}

{
  echo "## Compile benchmark results"
  echo
  echo "$runs runs per benchmark."
  echo
  echo "- Commit: \`$commit\`"
  echo "- Runner: \`$runner\`"
  echo "- Toolchain: \`$(rustc --version)\`"
  echo "- Hyperfine: \`$installed_hyperfine\`"
  echo "- LLVM lines: \`$installed_llvm_lines\`"
  echo
  cat "$timings"

  if [[ -n "$rss_unit" ]]; then
    echo
    echo "Maximum RSS (rough)."
    echo
    echo "| slice | maximum RSS |"
    echo "|---|---:|"

    for slice in "${slices[@]}"; do
      rss_mib=$(max_rss_mib "$compile_bench_target_root/$slice.max-rss")
      echo "| \`$slice\` | $rss_mib MiB |"
    done
  fi

  echo
  echo "### Binary size"
  echo
  echo "| slice | unstripped | stripped |"
  echo "|---|---:|---:|"

  for slice in "${slices[@]}"; do
    unstripped_bytes=$(<"$compile_bench_target_root/$slice.unstripped-bytes")
    stripped_bytes=$(<"$compile_bench_target_root/$slice.stripped-bytes")
    unstripped_mib=$(awk -v bytes="$unstripped_bytes" 'BEGIN { printf "%.2f", bytes / 1048576 }')
    stripped_mib=$(awk -v bytes="$stripped_bytes" 'BEGIN { printf "%.2f", bytes / 1048576 }')
    echo "| \`$slice\` | $unstripped_mib MiB | $stripped_mib MiB |"
  done

  echo
  echo "### LLVM IR"
  echo
  echo "\`cargo llvm-lines\` reports only the selected root crate; separately compiled dependencies are excluded."
  echo "\`async-stripe-shared\` is a standalone default-feature library measurement; \`bench-*\` rows measure consumer root crates."
  echo
  echo "| target | scope | lines | copies |"
  echo "|---|---|---:|---:|"

  llvm_targets=(async-stripe-shared)
  for slice in "${slices[@]}"; do
    llvm_targets+=("bench-$slice")
  done

  for target in "${llvm_targets[@]}"; do
    totals=$(llvm_totals "$llvm_lines_dir/$target.txt")
    lines=${totals%% *}
    copies=${totals##* }
    if [[ "$target" == async-stripe-shared ]]; then
      scope="library crate"
    else
      scope="consumer root"
    fi
    echo "| \`$target\` | $scope | $lines | $copies |"
  done

  for target in "${llvm_targets[@]}"; do
    echo
    echo "<details>"
    echo "<summary><code>$target</code> top IR producers</summary>"
    echo
    echo '```text'
    sed -n '1,11p' "$llvm_lines_dir/$target.txt"
    echo '```'
    echo "</details>"
  done
} > "$report"

echo
echo "Wrote $report"
echo
cat "$report"
