class Lune < Formula
  desc "Standalone Luau script runtime"
  homepage "https://lune-org.github.io/docs"
  url "https://github.com/filiptibell/lune/archive/refs/tags/v0.7.5.tar.gz"
  sha256 "e8191df5d6844026772cc7afab1083235a265c506474c4c4dee0a7724b04f775"
  license "MPL-2.0"

  livecheck do
    url :stable
    regex(/^v?(\d+(?:\.\d+)+)$/i)
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    (testpath/"test.lua").write("print(2 + 2)")
    output = shell_output("#{bin}/lune test.lua")
    assert_equal "4", output.chomp
  end
end
