class Rcm < Formula
  desc "Composable AI pipelines with .rcm files"
  homepage "https://github.com/EricSanchezok/recursive-context-machine"
  version "0.2.25"
  license "MIT"

  livecheck do
    url "https://github.com/EricSanchezok/recursive-context-machine/releases/latest"
    regex(%r{href=.*?/tag/v?(\d+(?:\.\d+)+)}i)
    strategy :page_match
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-darwin.tar.gz"
      sha256 "de6c60a533467d5a88f7d1364f31373b8ec68061ec2e4bcc11c7da11d2cf05f9"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-darwin.tar.gz"
      sha256 "3ae9b877df2c7a1e2bd5d055c392cbbf1332755d2d02a0815a18f26680b108d4"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-linux.tar.gz"
      sha256 "d69533927be1c8cc3d59f9488d87a65be969fd231189a21393d62768bc57645f"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-linux.tar.gz"
      sha256 "727299dd97c638c967a039711632a2ee7810eac990104d23a76b1489ab5bf75f"
    end
  end

  def install
    bin.install "accelerate"
  end

  test do
    system "#{bin}/accelerate", "--version"
  end
end
