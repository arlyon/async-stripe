#!/usr/bin/env bash
# Compile-time + binary-size bench for async-stripe.
#
# Runs each slice in bench/compile/slices/ through cargo check, cargo build,
# cargo build --release. Single run per measurement (no variance handling —
# compile times are long enough that noise is tolerable for diff purposes).
#
# Output:
#   - bench/compile/results/<sha>.json  machine-readable timings + sizes
#   - stdout                            markdown table; if bench/compile/baseline.json
#                                       exists, also prints % deltas vs baseline
#
# Typical workflow:
#   1. On master: ./bench/compile/run.sh && cp bench/compile/results/<sha>.json \
#                                            bench/compile/baseline.json
#   2. On branch: ./bench/compile/run.sh
#   3. Compare the delta table that prints.

set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
BENCH=$ROOT/bench/compile
SLICES_DIR=$BENCH/slices
RESULTS=$BENCH/results
mkdir -p "$RESULTS"

SHA=$(git -C "$ROOT" rev-parse --short HEAD)
DIRTY=""
if ! git -C "$ROOT" diff --quiet || ! git -C "$ROOT" diff --cached --quiet; then
  DIRTY="-dirty"
fi
OUT=$RESULTS/${SHA}${DIRTY}.json

# Start JSON array.
echo "[]" > "$OUT"

append_json() {
  local tmp
  tmp=$(mktemp)
  jq ". += [$1]" "$OUT" > "$tmp" && mv "$tmp" "$OUT"
}

time_cmd() {
  # Prints elapsed integer seconds. Ignores stdout of the command.
  local start=$SECONDS
  "$@" >/dev/null 2>&1
  echo $((SECONDS - start))
}

# Find a non-debug-info binary in a target profile dir. Requires one to exist.
find_bin() {
  local dir=$1 name=$2
  # Cargo produces the binary at target/<profile>/<package-name> (hyphens preserved).
  if [ -x "$dir/$name" ] && [ ! -d "$dir/$name" ]; then
    echo "$dir/$name"
    return
  fi
  # Fallback: first executable file that isn't dep metadata.
  find "$dir" -maxdepth 1 -type f -perm -u+x ! -name '*.d' ! -name '*.rmeta' | head -1
}

size_of() {
  stat -f%z "$1" 2>/dev/null || stat -c%s "$1"
}

for slice_dir in "$SLICES_DIR"/*/; do
  slice=$(basename "$slice_dir")
  manifest="$slice_dir/Cargo.toml"
  pkg_name=$(awk -F'"' '/^name =/ {print $2; exit}' "$manifest")

  echo "=== $slice ===" >&2

  # Each measurement runs in its own fresh target dir. Sharing one target across
  # the three measurements would let `cargo check` prime the typecheck cache for
  # `cargo build`, which confuses diffs if slices / bench sources change between
  # runs. Cold everywhere means the deltas are always honest.

  echo "  cargo check..." >&2
  TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
  export CARGO_TARGET_DIR=$TARGET
  t_check=$(time_cmd cargo check --manifest-path "$manifest")
  rm -rf "$TARGET"

  echo "  cargo build..." >&2
  TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
  export CARGO_TARGET_DIR=$TARGET
  t_build=$(time_cmd cargo build --manifest-path "$manifest")
  debug_bin=$(find_bin "$TARGET/debug" "$pkg_name")
  debug_size=$(size_of "$debug_bin")
  rm -rf "$TARGET"

  echo "  cargo build --release..." >&2
  TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
  export CARGO_TARGET_DIR=$TARGET
  t_release=$(time_cmd cargo build --release --manifest-path "$manifest")
  release_bin=$(find_bin "$TARGET/release" "$pkg_name")
  release_size=$(size_of "$release_bin")
  # Stripped size too, since that's closer to what gets shipped.
  strip "$release_bin" 2>/dev/null || true
  release_stripped=$(size_of "$release_bin")
  rm -rf "$TARGET"

  append_json "$(jq -n \
    --arg slice "$slice" \
    --argjson check $t_check \
    --argjson build $t_build \
    --argjson release $t_release \
    --argjson debug_size $debug_size \
    --argjson release_size $release_size \
    --argjson release_stripped $release_stripped \
    '{slice:$slice, check_s:$check, build_s:$build, release_s:$release,
      debug_bytes:$debug_size, release_bytes:$release_size, release_stripped_bytes:$release_stripped}')"
done

echo >&2
echo "Wrote $OUT" >&2
echo >&2

# Print a markdown table of current results, and deltas vs baseline if present.
BASELINE=$BENCH/baseline.json
if [ -f "$BASELINE" ]; then
  echo "| slice | check (s) | build (s) | release (s) | debug (MB) | release (MB) | stripped (MB) |"
  echo "|-------|-----------|-----------|-------------|------------|--------------|---------------|"
  jq -r --slurpfile base "$BASELINE" '
    def mb(b): (b / 1048576 * 10 | round) / 10;
    def fmt_delta(cur; bas):
      if bas == 0 then "\(cur)"
      else "\(cur) (\(((cur - bas) / bas * 100) | . * 10 | round / 10 | if . > 0 then "+\(.)%" else "\(.)%" end))"
      end;
    def fmt_delta_mb(cur; bas):
      if bas == 0 then "\(mb(cur))"
      else "\(mb(cur)) (\(((cur - bas) / bas * 100) | . * 10 | round / 10 | if . > 0 then "+\(.)%" else "\(.)%" end))"
      end;
    .[] as $c |
    ($base[0] // []) | map(select(.slice == $c.slice)) | .[0] as $b |
    if $b == null then
      "| \($c.slice) | \($c.check_s) | \($c.build_s) | \($c.release_s) | \(mb($c.debug_bytes)) | \(mb($c.release_bytes)) | \(mb($c.release_stripped_bytes)) |"
    else
      "| \($c.slice) | \(fmt_delta($c.check_s; $b.check_s)) | \(fmt_delta($c.build_s; $b.build_s)) | \(fmt_delta($c.release_s; $b.release_s)) | \(fmt_delta_mb($c.debug_bytes; $b.debug_bytes)) | \(fmt_delta_mb($c.release_bytes; $b.release_bytes)) | \(fmt_delta_mb($c.release_stripped_bytes; $b.release_stripped_bytes)) |"
    end
  ' "$OUT"
else
  echo "(No baseline.json — copy this result there to establish one.)"
  echo
  echo "| slice | check (s) | build (s) | release (s) | debug (MB) | release (MB) | stripped (MB) |"
  echo "|-------|-----------|-----------|-------------|------------|--------------|---------------|"
  jq -r '
    def mb(b): (b / 1048576 * 10 | round) / 10;
    .[] | "| \(.slice) | \(.check_s) | \(.build_s) | \(.release_s) | \(mb(.debug_bytes)) | \(mb(.release_bytes)) | \(mb(.release_stripped_bytes)) |"
  ' "$OUT"
fi
