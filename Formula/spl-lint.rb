class SplLint < Formula
  desc "A linter for Splunk Search Processing Language (SPL)"
  homepage "https://github.com/hiboma/spl-lint"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/hiboma/spl-lint/releases/download/v#{version}/spl-lint-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    else
      url "https://github.com/hiboma/spl-lint/releases/download/v#{version}/spl-lint-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    url "https://github.com/hiboma/spl-lint/releases/download/v#{version}/spl-lint-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "PLACEHOLDER"
  end

  def install
    bin.install "spl-lint"
  end

  test do
    output = pipe_output("#{bin}/spl-lint", "index=main | stats count by src_ip", 0)
    assert_equal "", output.strip
  end
end
