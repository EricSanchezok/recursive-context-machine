class Rcm < Formula
  desc "Composable AI pipelines with .rcm files"
  homepage "https://github.com/EricSanchezok/recursive-context-machine"
  version "0.2.20"
  license "MIT"

  livecheck do
    url "https://github.com/EricSanchezok/recursive-context-machine/releases/latest"
    regex(%r{href=.*?/tag/v?(\d+(?:\.\d+)+)}i)
    strategy :page_match
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-darwin.tar.gz"
      sha256 "60d18859bbd4b136d560335bae0930715b92fec5d8bb57be66710aa068c0b50b"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-darwin.tar.gz"
      sha256 "612f73aba00d279fa1c77f0d6646735454613b0607fb75112358d67549f4a78c"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-linux.tar.gz"
      sha256 "d2be547d5e39b5280b0a4f716f6151aa86db1cb785f678adfc6f38ec5dde07a5"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-linux.tar.gz"
      sha256 "2ed7325911d24287a7acc0f1081e008b70af9b92aee7a12a9464165fb247e3bb"
    end
  end

  def install
    bin.install "accelerate"
  end

  test do
    system "#{bin}/accelerate", "--version"
  end
end
