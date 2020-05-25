#! /bin/sh
set -o errexit -o nounset

publish() (
  echo "::group::Crate: $1"
  echo '::group::building...'
  cargo build --locked --release
  echo '::endgroup::'
  echo '::group::publishing...'
  cd ./crates/"$1"
  cargo publish
  echo '::endgroup::'
  echo '::endgroup::'
)

publish pretty-exec-lib
publish pretty-exec
