#! /bin/bash
set -o errexit -o nounset

if [ -z "$RELEASE_TAG" ]; then
  echo '::error::Environment variable RELEASE_TAG is required but missing'
  exit 1
fi

wait_for_version() (
  sleep 1
  echo .
  prefix=https://raw.githubusercontent.com/rust-lang/crates.io-index/master/pr/et
  while read -r json
  do
    tag=$(echo "$json" | jq --raw-output '.vers')
    if [ "$tag" = "$RELEASE_TAG" ]; then
      exit 0
    fi
  done < <(curl -fsSL $prefix/"$1")
  wait_for_version "$1"
)

publish() (
  echo "::group::Publishing $1..."
  cd ./crates/"$1"
  cargo publish
  echo '::endgroup::'
  echo "::group::Waiting for $1 to be available..."
  wait_for_version "$1"
  echo '::endgroup::'
)

publish pretty-exec-lib
publish pretty-exec
