#!/bin/sh
set -eu

REPO="EricSanchezok/recursive-context-machine"
BASE_URL="https://github.com/${REPO}/releases"
BIN_NAME="accelerate"

bold()  { printf '\033[1m%s\033[0m\n' "$*"; }
red()   { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }

detect_platform() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Linux)  os="linux" ;;
    Darwin) os="darwin" ;;
    MINGW*|MSYS*|CYGWIN*) os="windows" ;;
    *)
      red "Unsupported OS: $OS"
      exit 1
      ;;
  esac

  case "$ARCH" in
    x86_64|amd64)   arch="x86_64" ;;
    aarch64|arm64)  arch="aarch64" ;;
    *)
      red "Unsupported architecture: $ARCH"
      exit 1
      ;;
  esac

  PLATFORM="${arch}-${os}"
}

resolve_version() {
  VERSION="${RCM_VERSION:-}"
  if [ -z "$VERSION" ]; then
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | grep '"tag_name"' \
      | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
    if [ -z "$VERSION" ]; then
      red "Unable to find latest release. Set RCM_VERSION=vX.Y.Z to pin a version."
      exit 1
    fi
  fi
}

install_binary() {
  if [ "$os" = "windows" ]; then
    ARCHIVE="${BIN_NAME}-${PLATFORM}.zip"
  else
    ARCHIVE="${BIN_NAME}-${PLATFORM}.tar.gz"
  fi
  DOWNLOAD_URL="${BASE_URL}/download/${VERSION}/${ARCHIVE}"

  bold "Downloading ${ARCHIVE} ..."
  TMPDIR="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR"' EXIT

  curl -fsSL "$DOWNLOAD_URL" -o "${TMPDIR}/${ARCHIVE}"

  if [ "$os" = "windows" ]; then
    unzip -q "${TMPDIR}/${ARCHIVE}" -d "$TMPDIR"
  else
    tar xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"
  fi

  if [ "$os" = "windows" ]; then
    INSTALL_DIR="${RCM_INSTALL_DIR:-$HOME/.rcm/bin}"
  elif [ -w /usr/local/bin ]; then
    INSTALL_DIR="${RCM_INSTALL_DIR:-/usr/local/bin}"
  else
    INSTALL_DIR="${RCM_INSTALL_DIR:-$HOME/.local/bin}"
  fi

  mkdir -p "$INSTALL_DIR"
  install -m 755 "${TMPDIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"

  green "Installed ${BIN_NAME} → ${INSTALL_DIR}/${BIN_NAME}"
}

main() {
  detect_platform
  resolve_version
  install_binary

  bold ""
  bold "Done! Run '${BIN_NAME} --help' to get started."
  bold ""

  case ":$PATH:" in
    *:"$INSTALL_DIR":*) ;;
    *)
      bold "Add ${INSTALL_DIR} to your PATH:"
      bold "  export PATH=\"${INSTALL_DIR}:\$PATH\""
      bold ""
      ;;
  esac
}

main
