class Lune < Formula
  LUNE_VERSION = "0.7.5"

  desc "Standalone Luau script runtime 🌙"
  homepage "https://lune-org.github.io/docs"
  url "https://github.com/filiptibell/lune/releases/download/v#{LUNE_VERSION}/lune-#{LUNE_VERSION}-macos-x86_64.zip"
  sha256 "3c9cb328d7c3049354c714021ba857ce83ded3da4ac4b4a2a046fb2116e0bd2e"
  license "MPL-2.0"

  def install
    bin.install "lune"
  end

  test do
    system "false"
  end
end
