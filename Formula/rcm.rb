class Rcm < Formula
  desc "Composable AI pipelines with .rcm files"
  homepage "https://github.com/EricSanchezok/recursive-context-machine"
  version "0.2.21"
  license "MIT"

  livecheck do
    url "https://github.com/EricSanchezok/recursive-context-machine/releases/latest"
    regex(%r{href=.*?/tag/v?(\d+(?:\.\d+)+)}i)
    strategy :page_match
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-darwin.tar.gz"
      sha256 "35660cb45d50f41687443289f4addaf59ec31a8d49a6c9cf38d0c42527a7defe"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-darwin.tar.gz"
      sha256 "cd7f59bf3f520e29826fbab576093a0d704b706fe7077b2ef0634f6dff857fce"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-aarch64-linux.tar.gz"
      sha256 "3fa79ed245d5ae712bfaf9abbc7a0ac749e4c600234fce7d2d35c738b5f6dd4c"
    else
      url "https://github.com/EricSanchezok/recursive-context-machine/releases/download/v#{version}/accelerate-x86_64-linux.tar.gz"
      sha256 "1df945a49c7980b13ae28c1f3019e541e8f009cb5731471b0b51242dfa14c2d7"
    end
  end

  def install
    bin.install "accelerate"
  end

  test do
    system "#{bin}/accelerate", "--version"
  end
end
