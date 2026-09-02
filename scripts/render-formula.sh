#!/bin/sh
set -eu

if [ "$#" -ne 6 ]; then
  printf 'usage: %s VERSION SHA256_DARWIN_ARM SHA256_DARWIN_X86 SHA256_LINUX_ARM SHA256_LINUX_X86 OUTPUT\n' "$0" >&2
  exit 2
fi

VERSION=$1
SHA256_DARWIN_ARM=$2
SHA256_DARWIN_X86=$3
SHA256_LINUX_ARM=$4
SHA256_LINUX_X86=$5
OUTPUT=$6

case "$VERSION" in
  ''|*[!0-9.]*|.*|*.)
    printf 'invalid version: %s\n' "$VERSION" >&2
    exit 1
    ;;
esac

validate_hash() {
  HASH=$1
  case "$HASH" in
    [!0-9a-fA-F]*|*[!0-9a-fA-F]*)
      return 1
      ;;
  esac
  [ "$(printf '%s' "$HASH" | awk '{ print length }')" -eq 64 ]
}

validate_hash "$SHA256_DARWIN_ARM" || { printf 'invalid darwin arm sha256\n' >&2; exit 1; }
validate_hash "$SHA256_DARWIN_X86" || { printf 'invalid darwin x86 sha256\n' >&2; exit 1; }
validate_hash "$SHA256_LINUX_ARM" || { printf 'invalid linux arm sha256\n' >&2; exit 1; }
validate_hash "$SHA256_LINUX_X86" || { printf 'invalid linux x86 sha256\n' >&2; exit 1; }

mkdir -p "$(dirname "$OUTPUT")"
cat > "$OUTPUT" <<EOF
class Rcm < Formula
  desc "Composable AI pipelines with .rcm files"
  homepage "https://github.com/EricSanchezok/recursive-context-machine"
  version "$VERSION"
  license "MIT"

  livecheck do
    url "https://github.com/EricSanchezok/recursive-context-machine/releases/latest"
    regex(%r{href=.*?/tag/v?(\\d+(?:\\.\\d+)+)}i)
    strategy :page_match
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-darwin.tar.gz"
      sha256 "$SHA256_DARWIN_ARM"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-darwin.tar.gz"
      sha256 "$SHA256_DARWIN_X86"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-linux.tar.gz"
      sha256 "$SHA256_LINUX_ARM"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-linux.tar.gz"
      sha256 "$SHA256_LINUX_X86"
    end
  end

  def install
    bin.install "accelerate"
  end

  test do
    system "#{bin}/accelerate", "--version"
  end
end
EOF
