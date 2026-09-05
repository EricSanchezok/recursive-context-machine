class Rcm < Formula
  desc "Composable AI pipelines with .rcm files"
  homepage "https://github.com/EricSanchezok/recursive-context-machine"
  version "0.2.24"
  license "MIT"

  livecheck do
    url "https://github.com/EricSanchezok/recursive-context-machine/releases/latest"
    regex(%r{href=.*?/tag/v?(\d+(?:\.\d+)+)}i)
    strategy :page_match
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-darwin.tar.gz"
      sha256 "2cc19b4c6d309d3eb47631c51f872d2c72488e9af546599c5c109f7df9c1260d"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-darwin.tar.gz"
      sha256 "9be3f3abdcdd4153bd643ea15f4d6bc20c42413eff73fef0dfa95401a0e4bf9d"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-linux.tar.gz"
      sha256 "12f9668224dce2b1cadc8570ef7ae3b88b0b8faf2cc3bddabe59152c97628c59"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-linux.tar.gz"
      sha256 "bbfc081691507672b70de8f2c20cc2b71470080cdcb78a3ca31cb846dd2bb347"
    end
  end

  def install
    bin.install "accelerate"
  end

  test do
    system "#{bin}/accelerate", "--version"
  end
end
