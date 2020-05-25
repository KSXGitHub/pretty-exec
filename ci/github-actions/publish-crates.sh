#! /bin/sh
set -o errexit -o nounset

publish() (
  echo "::group::Publishing $1..."
  cd ./crates/"$1"
  cargo publish
  echo '::endgroup::'
)

publish pretty-exec-lib
publish pretty-exec
