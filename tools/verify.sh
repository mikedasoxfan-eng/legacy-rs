#!/usr/bin/env bash
# Compare the Rust generator against the Java oracle across a fixed set of seeds/regions.
# Usage: tools/verify.sh            (exit code 0 only if every case is an exact match)
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
oracle="$root/tools/oracle/oracle.sh"
out="$root/tools/oracle/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen || exit 1
cmp="$root/target/release/compare"

fail=0
"$oracle" sin sin.bin && "$cmp" sin "$out/sin.bin" || fail=1

# seed  x0 z0 x1 z1   label
cases=(
  "0                    -8 -8 7 7          origin"
  "3257840388504953787  -6 -6 5 5          pack.png seed"
  "-4172144997902289642 -4 -4 3 3          negative seed"
  "123456789            100 -300 107 -293  mid-range"
  "42                   784420 0 784431 3  far lands x"
  "42                   -3 -784431 0 -784420  far lands -z"
  "9223372036854775807  -2 -2 2 2          max long seed"
)
for c in "${cases[@]}"; do
  read -r seed x0 z0 x1 z1 label <<<"$c"
  f="raw_${seed}_${x0}_${z0}.bin"
  echo "== $label (seed $seed, chunks $x0,$z0..$x1,$z1)"
  "$oracle" raw "$seed" "$x0" "$z0" "$x1" "$z1" "$f" && "$cmp" raw "$seed" "$out/$f" || fail=1
done
[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail
