#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
setup_script=""

# Prefer rustup's toolchain over an older distro Cargo. The repository lockfile
# requires the current Rust toolchain, and this keeps the command independent of
# the caller's PATH ordering.
if [[ -x "${HOME:-}/.cargo/bin/cargo" ]]; then
  export PATH="${HOME}/.cargo/bin:$PATH"
fi

# A caller may already have selected an SDK. Otherwise discover the standard
# per-user install used by LunarG, plus the common system locations.
if [[ -n "${VULKAN_SDK:-}" && -f "${VULKAN_SDK%/}/../setup-env.sh" ]]; then
  setup_script="${VULKAN_SDK%/}/../setup-env.sh"
elif [[ -n "${HOME:-}" ]]; then
  setup_script="$(find "$HOME/vulkan" /opt/vulkan /usr/local/vulkan \
    -maxdepth 4 -type f -name setup-env.sh 2>/dev/null | sort -V | tail -n 1 || true)"
fi

if [[ -n "$setup_script" ]]; then
  # shellcheck disable=SC1090
  source "$setup_script"
fi

if [[ -z "${VULKAN_SDK:-}" || -z "${CMAKE_PREFIX_PATH:-}" ]]; then
  cat >&2 <<'EOF'
Talk's Linux Vulkan build needs the Vulkan SDK.
Install it under ~/vulkan, or source its setup-env.sh before running this command.
EOF
  exit 1
fi

# whisper-rs-sys keeps a CMake cache under Cargo's target directory. If it was
# configured before the SDK was installed, CMake will keep using the old system
# headers until that package build is refreshed.
cache_marker="$repo_dir/src-tauri/target/.talk-vulkan-sdk"
sdk_key="$VULKAN_SDK|$CMAKE_PREFIX_PATH"
if [[ ! -f "$cache_marker" || "$(<"$cache_marker")" != "$sdk_key" ]]; then
  sdk_include="${VULKAN_SDK%/}/include"
  stale_cache=""
  while IFS= read -r cache; do
    cached_include="$(sed -n 's/^Vulkan_INCLUDE_DIR:PATH=//p' "$cache" | head -n 1)"
    if [[ -n "$cached_include" && "$cached_include" != "$sdk_include" ]]; then
      stale_cache="$cache"
      break
    fi
  done < <(find "$repo_dir/src-tauri/target" -type f -path \
    '*/whisper-rs-sys-*/out/build/CMakeCache.txt' 2>/dev/null)

  if [[ -n "$stale_cache" ]]; then
    cargo clean --manifest-path "$repo_dir/src-tauri/Cargo.toml" -p whisper-rs-sys
  fi
  mkdir -p "$(dirname "$cache_marker")"
  printf '%s' "$sdk_key" > "$cache_marker"
fi

if [[ "${1:-}" == "cargo" ]]; then
  shift
  exec cargo "$@"
fi

exec "$repo_dir/node_modules/.bin/tauri" "$@"
