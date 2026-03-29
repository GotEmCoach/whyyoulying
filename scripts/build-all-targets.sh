#!/bin/bash
# build-all-targets.sh — Build whyyoulying for every supported architecture.
# Outputs: release/whyyoulying-<target>[.exe|.wasm]
#
# Native (Mac Mini):     aarch64-apple-darwin, x86_64-apple-darwin,
#                        aarch64-apple-ios (lib), wasm32-unknown-unknown (lib)
# Remote (st/gd/lf):    x86_64-unknown-linux-gnu
# Cross (needs docker):  aarch64-unknown-linux-gnu, armv7-unknown-linux-gnueabihf,
#                        riscv64gc-unknown-linux-gnu, x86_64-pc-windows-gnu,
#                        x86_64-unknown-freebsd, powerpc64le-unknown-linux-gnu
# Android (needs NDK):   aarch64-linux-android

set -uo pipefail
cd "$(dirname "$0")/.."

PROJECT="whyyoulying"
RELEASE_DIR="release"
mkdir -p "$RELEASE_DIR"

GREEN="\033[32m"
RED="\033[31m"
YELLOW="\033[33m"
RESET="\033[0m"

built=0
failed=0
skipped=0

build_target() {
    local target="$1"
    local method="$2"  # native, remote, cross, ndk
    local suffix=""
    local bin_name="$PROJECT"

    [[ "$target" == *windows* ]] && suffix=".exe"
    [[ "$target" == *wasm* ]] && suffix=".wasm"

    echo -n "  $target ($method): "

    case "$method" in
        native)
            if [[ "$target" == *wasm* ]] || [[ "$target" == *ios* ]]; then
                # lib-only targets
                if cargo build --release --target "$target" --lib 2>/dev/null; then
                    if [[ "$target" == *wasm* ]]; then
                        local src="target/$target/release/lib${PROJECT}.wasm"
                        [ -f "$src" ] && cp "$src" "$RELEASE_DIR/${PROJECT}-${target}${suffix}"
                    fi
                    echo -e "${GREEN}OK (lib)${RESET}"
                    ((built++))
                else
                    echo -e "${RED}FAILED${RESET}"
                    ((failed++))
                fi
            else
                if cargo build --release --target "$target" 2>/dev/null; then
                    cp "target/$target/release/${bin_name}${suffix}" "$RELEASE_DIR/${PROJECT}-${target}${suffix}"
                    echo -e "${GREEN}OK $(du -h "$RELEASE_DIR/${PROJECT}-${target}${suffix}" | cut -f1)${RESET}"
                    ((built++))
                else
                    echo -e "${RED}FAILED${RESET}"
                    ((failed++))
                fi
            fi
            ;;
        remote)
            local host="$3"
            echo -n "(via $host) "
            # Check if host is reachable
            if ! ssh -o ConnectTimeout=5 "$host" "true" 2>/dev/null; then
                echo -e "${YELLOW}SKIP (host unreachable)${RESET}"
                ((skipped++))
                return
            fi
            # Rsync source, build remotely
            rsync -az --exclude target --exclude .git . "$host:~/${PROJECT}-build/" 2>/dev/null
            ssh "$host" "echo '[workspace]' >> ~/${PROJECT}-build/Cargo.toml; \
                source ~/.cargo/env 2>/dev/null; \
                cd ~/${PROJECT}-build && cargo build --release 2>&1 | tail -1" 2>/dev/null
            if scp "$host:~/${PROJECT}-build/target/release/${bin_name}" "$RELEASE_DIR/${PROJECT}-${target}" 2>/dev/null; then
                echo -e "${GREEN}OK $(du -h "$RELEASE_DIR/${PROJECT}-${target}" | cut -f1)${RESET}"
                ((built++))
            else
                echo -e "${RED}FAILED${RESET}"
                ((failed++))
            fi
            ;;
        cross)
            if command -v cross &>/dev/null; then
                if cross build --release --target "$target" 2>/dev/null; then
                    cp "target/$target/release/${bin_name}${suffix}" "$RELEASE_DIR/${PROJECT}-${target}${suffix}"
                    echo -e "${GREEN}OK $(du -h "$RELEASE_DIR/${PROJECT}-${target}${suffix}" | cut -f1)${RESET}"
                    ((built++))
                else
                    echo -e "${RED}FAILED${RESET}"
                    ((failed++))
                fi
            else
                echo -e "${YELLOW}SKIP (cross not installed — cargo install cross)${RESET}"
                ((skipped++))
            fi
            ;;
        ndk)
            if command -v cargo-ndk &>/dev/null; then
                if cargo ndk -t arm64-v8a build --release --lib 2>/dev/null; then
                    echo -e "${GREEN}OK (lib)${RESET}"
                    ((built++))
                else
                    echo -e "${RED}FAILED${RESET}"
                    ((failed++))
                fi
            else
                echo -e "${YELLOW}SKIP (cargo-ndk not installed)${RESET}"
                ((skipped++))
            fi
            ;;
    esac
}

echo "============================================================"
echo "  $PROJECT — multi-architecture build"
echo "============================================================"
echo ""
echo "Native targets (Mac Mini):"
build_target "aarch64-apple-darwin"       native
build_target "x86_64-apple-darwin"        native
build_target "aarch64-apple-ios"          native
build_target "wasm32-unknown-unknown"     native

echo ""
echo "Remote targets (worker nodes):"
build_target "x86_64-unknown-linux-gnu"   remote st

echo ""
echo "Cross-compile targets (need 'cross' + Docker):"
build_target "aarch64-unknown-linux-gnu"         cross
build_target "armv7-unknown-linux-gnueabihf"     cross
build_target "riscv64gc-unknown-linux-gnu"       cross
build_target "x86_64-pc-windows-gnu"             cross
build_target "x86_64-unknown-freebsd"            cross
build_target "powerpc64le-unknown-linux-gnu"     cross

echo ""
echo "Android (need cargo-ndk + NDK):"
build_target "aarch64-linux-android"      ndk

echo ""
echo "============================================================"
echo "  Results: ${built} built, ${failed} failed, ${skipped} skipped"
echo "============================================================"
echo ""
ls -lh "$RELEASE_DIR"/ 2>/dev/null

exit $failed
