# typed: false
# frozen_string_literal: true

# This file was generated by GoReleaser. DO NOT EDIT.
class TknWatch < Formula
  desc "tkn-watch - watch tekton pipelinerun execution"
  homepage "https://github.com/chmouel/tkn-watch"
  version "0.3.0"

  on_macos do
    url "https://github.com/chmouel/tkn-watch/releases/download/0.3.0/tkn-watch_0.3.0_macOS_all.tar.gz"
    sha256 "40034b87f9a14af7bf8020713e68364e7c1055b01a3a6c03e767f08bef38953e"

    def install
      bin.install "tkn-watch" => "tkn-watch"
      prefix.install_metafiles
      bash_completion.install "completions/tkn-watch.bash"
      fish_completion.install "completions/tkn-watch.fish"
      zsh_completion.install "completions/_tkn-watch"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/chmouel/tkn-watch/releases/download/0.3.0/tkn-watch_0.3.0_linux_amd64.tar.gz"
      sha256 "67267a8c642d83dfd72e7a64c2c07257da3d336fa36716925882135b33ea5c9b"

      def install
        bin.install "tkn-watch" => "tkn-watch"
        prefix.install_metafiles
        bash_completion.install "completions/tkn-watch.bash"
        fish_completion.install "completions/tkn-watch.fish"
        zsh_completion.install "completions/_tkn-watch"
      end
    end
  end
end
