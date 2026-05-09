#!/usr/bin/env bash
# Compile-time + binary-size bench for async-stripe.
#
# Runs each slice in bench/compile/slices/ through cargo check, cargo build,
# cargo build --release. Single run per measurement (no variance handling —
# compile times are long enough that noise is tolerable for diff purposes).
# Captures peak RSS via /usr/bin/time -l (macOS).
#
# Usage:
#   ./run.sh                       all slices, parallel-rustc only
#   ./run.sh deser-many minimal    selected slices only
#   ./run.sh --j1                  all slices + extra `cargo build --release -j 1`
#                                  pass for true single-process peak RSS
#   ./run.sh --j1 deser-many       selected slice with -j 1 pass
#
# Both passes report `ru_maxrss` from wait3 (cargo + descendants, max-of-any).
# Neither gives the true concurrent total — that would require sampling. What
# you get instead:
#   - parallel: largest single-process spike during a realistic parallel build.
#               Often higher than -j1 because workers contend and hold onto
#               working sets longer under pressure.
#   - -j1:      largest rustc spike with no inter-process contention.
# Both are interesting; -j1 is opt-in because it's slow (no parallelism).
#
# Output:
#   - bench/compile/results/<sha>.json  machine-readable timings + sizes + peak RSS
#   - stdout                            markdown table; if bench/compile/baseline.json
#                                       exists, also prints % deltas vs baseline

set -euo pipefail

MEASURE_J1=false
SLICE_FILTER=()
for arg in "$@"; do
  case "$arg" in
    --j1) MEASURE_J1=true ;;
    -*)   echo "unknown flag: $arg" >&2; exit 2 ;;
    *)    SLICE_FILTER+=("$arg") ;;
  esac
done

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

time_and_mem_cmd() {
  # Prints "<elapsed_seconds> <peak_rss_bytes>" to stdout. Ignores command stdout.
  # /usr/bin/time -l writes its stats to stderr alongside the command's stderr; we
  # capture both and parse out "maximum resident set size" (in bytes on Darwin).
  local start=$SECONDS
  local stderr_file
  stderr_file=$(mktemp)
  /usr/bin/time -l "$@" >/dev/null 2>"$stderr_file"
  local elapsed=$((SECONDS - start))
  local rss
  rss=$(awk '/maximum resident set size/ {print $1; exit}' "$stderr_file")
  rm -f "$stderr_file"
  printf "%s %s\n" "$elapsed" "${rss:-0}"
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

slice_in_filter() {
  local s=$1
  if [ ${#SLICE_FILTER[@]} -eq 0 ]; then return 0; fi
  for f in "${SLICE_FILTER[@]}"; do
    [ "$f" = "$s" ] && return 0
  done
  return 1
}

for slice_dir in "$SLICES_DIR"/*/; do
  slice=$(basename "$slice_dir")
  if ! slice_in_filter "$slice"; then continue; fi
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
  read t_check rss_check < <(time_and_mem_cmd cargo check --manifest-path "$manifest")
  rm -rf "$TARGET"

  echo "  cargo build..." >&2
  TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
  export CARGO_TARGET_DIR=$TARGET
  read t_build rss_build < <(time_and_mem_cmd cargo build --manifest-path "$manifest")
  debug_bin=$(find_bin "$TARGET/debug" "$pkg_name")
  debug_size=$(size_of "$debug_bin")
  rm -rf "$TARGET"

  echo "  cargo build --release..." >&2
  TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
  export CARGO_TARGET_DIR=$TARGET
  read t_release rss_release < <(time_and_mem_cmd cargo build --release --manifest-path "$manifest")
  release_bin=$(find_bin "$TARGET/release" "$pkg_name")
  release_size=$(size_of "$release_bin")
  # Stripped size too, since that's closer to what gets shipped.
  strip "$release_bin" 2>/dev/null || true
  release_stripped=$(size_of "$release_bin")
  rm -rf "$TARGET"

  t_release_j1=null
  rss_release_j1=null
  if $MEASURE_J1; then
    echo "  cargo build --release -j 1..." >&2
    TARGET=$(mktemp -d -t async-stripe-bench.XXXX)
    export CARGO_TARGET_DIR=$TARGET
    read t_release_j1 rss_release_j1 < <(time_and_mem_cmd cargo build --release -j 1 --manifest-path "$manifest")
    rm -rf "$TARGET"
  fi

  append_json "$(jq -n \
    --arg slice "$slice" \
    --argjson check $t_check \
    --argjson build $t_build \
    --argjson release $t_release \
    --argjson check_rss $rss_check \
    --argjson build_rss $rss_build \
    --argjson release_rss $rss_release \
    --argjson release_j1 $t_release_j1 \
    --argjson release_j1_rss $rss_release_j1 \
    --argjson debug_size $debug_size \
    --argjson release_size $release_size \
    --argjson release_stripped $release_stripped \
    '{slice:$slice, check_s:$check, build_s:$build, release_s:$release,
      check_rss_bytes:$check_rss, build_rss_bytes:$build_rss, release_rss_bytes:$release_rss,
      release_j1_s:$release_j1, release_j1_rss_bytes:$release_j1_rss,
      debug_bytes:$debug_size, release_bytes:$release_size, release_stripped_bytes:$release_stripped}')"
done

echo >&2
echo "Wrote $OUT" >&2
echo >&2

# Print a markdown table of current results, and deltas vs baseline if present.
# Peak RSS is reported in MiB. The "release peak RSS" column shows the parallel
# build's max-of-any child-process spike. The "release -j1 peak RSS" column,
# when populated, shows the same metric on a serialized build (no contention).
BASELINE=$BENCH/baseline.json
HAS_J1=$(jq 'any(.release_j1_s != null)' "$OUT")

# Build column header dynamically based on whether any record has j1 data.
print_header() {
  if [ "$HAS_J1" = "true" ]; then
    echo "| slice | check (s) | build (s) | release (s) | release peak RSS (MiB) | release -j1 (s) | release -j1 peak RSS (MiB) | debug (MB) | release (MB) | stripped (MB) |"
    echo "|-------|-----------|-----------|-------------|------------------------|-----------------|----------------------------|------------|--------------|---------------|"
  else
    echo "| slice | check (s) | build (s) | release (s) | release peak RSS (MiB) | debug (MB) | release (MB) | stripped (MB) |"
    echo "|-------|-----------|-----------|-------------|------------------------|------------|--------------|---------------|"
  fi
}

print_header

if [ -f "$BASELINE" ]; then
  jq -r --slurpfile base "$BASELINE" --argjson has_j1 "$HAS_J1" '
    def mb(b): (b / 1048576 * 10 | round) / 10;
    def fmt(cur; bas):
      if bas == null or bas == 0 then "\(cur)"
      else "\(cur) (\(((cur - bas) / bas * 100) | . * 10 | round / 10 | if . > 0 then "+\(.)%" else "\(.)%" end))"
      end;
    def fmt_mb(cur; bas):
      if cur == null then "—"
      elif bas == null or bas == 0 then "\(mb(cur))"
      else "\(mb(cur)) (\(((cur - bas) / bas * 100) | . * 10 | round / 10 | if . > 0 then "+\(.)%" else "\(.)%" end))"
      end;
    def fmt_opt(cur; bas):
      if cur == null then "—"
      elif bas == null or bas == 0 then "\(cur)"
      else "\(cur) (\(((cur - bas) / bas * 100) | . * 10 | round / 10 | if . > 0 then "+\(.)%" else "\(.)%" end))"
      end;
    .[] as $c |
    ($base[0] // []) | map(select(.slice == $c.slice)) | .[0] as $b |
    if $has_j1 then
      "| \($c.slice) | \(fmt($c.check_s; ($b.check_s // null))) | \(fmt($c.build_s; ($b.build_s // null))) | \(fmt($c.release_s; ($b.release_s // null))) | \(fmt_mb($c.release_rss_bytes; ($b.release_rss_bytes // null))) | \(fmt_opt($c.release_j1_s; ($b.release_j1_s // null))) | \(fmt_mb($c.release_j1_rss_bytes; ($b.release_j1_rss_bytes // null))) | \(fmt_mb($c.debug_bytes; ($b.debug_bytes // null))) | \(fmt_mb($c.release_bytes; ($b.release_bytes // null))) | \(fmt_mb($c.release_stripped_bytes; ($b.release_stripped_bytes // null))) |"
    else
      "| \($c.slice) | \(fmt($c.check_s; ($b.check_s // null))) | \(fmt($c.build_s; ($b.build_s // null))) | \(fmt($c.release_s; ($b.release_s // null))) | \(fmt_mb($c.release_rss_bytes; ($b.release_rss_bytes // null))) | \(fmt_mb($c.debug_bytes; ($b.debug_bytes // null))) | \(fmt_mb($c.release_bytes; ($b.release_bytes // null))) | \(fmt_mb($c.release_stripped_bytes; ($b.release_stripped_bytes // null))) |"
    end
  ' "$OUT"
else
  echo "(No baseline.json — copy this result there to establish one.)"
  jq -r --argjson has_j1 "$HAS_J1" '
    def mb(b): (b / 1048576 * 10 | round) / 10;
    def opt(v): if v == null then "—" else "\(v)" end;
    def opt_mb(v): if v == null then "—" else "\(mb(v))" end;
    .[] |
    if $has_j1 then
      "| \(.slice) | \(.check_s) | \(.build_s) | \(.release_s) | \(mb(.release_rss_bytes)) | \(opt(.release_j1_s)) | \(opt_mb(.release_j1_rss_bytes)) | \(mb(.debug_bytes)) | \(mb(.release_bytes)) | \(mb(.release_stripped_bytes)) |"
    else
      "| \(.slice) | \(.check_s) | \(.build_s) | \(.release_s) | \(mb(.release_rss_bytes)) | \(mb(.debug_bytes)) | \(mb(.release_bytes)) | \(mb(.release_stripped_bytes)) |"
    end
  ' "$OUT"
fi
