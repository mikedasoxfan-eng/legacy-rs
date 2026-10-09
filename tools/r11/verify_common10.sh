#!/usr/bin/env bash
# Rust diagnostic files are hard-coded under target/ in the inherited binaries.
# Isolate their working directory and record first-difference evidence per case.
r10_compare() { python "$root/tools/r11/compare10.py" "$@"; }
r10_oracle() {
  "$@"
  local status=$?
  if [ "$status" != 0 ]; then echo "ORACLE FAILED: $* (exit $status)" >&2; fi
  return "$status"
}

# Captures need their configured asset directory. Pure comparison modes can use
# an isolated working directory, including inherited client diagnostic paths.
r10_client() {
  local binary="$1"; shift
  case " $* " in
    *" --guishot "*|*" --shot "*) "$binary" "$@" ;;
    *) r10_compare "$binary" "$@" ;;
  esac
}
