#! /bin/bash
set -o errexit -o nounset

if [ -z "$RELEASE_TAG" ]; then
	echo '::error::Environment variable RELEASE_TAG is required but missing'
	exit 1
fi

wait_for_version() (
	prefix=https://raw.githubusercontent.com/rust-lang/crates.io-index/master/pr/et
	url=$prefix/"$1?without-cache-$(date +%s)"
	echo "url: $url"

	printf '60 seconds'
	for _ in {0..59}; do
		sleep 1
		printf .
	done
	echo

	while read -r json; do
		vers=$(echo "$json" | jq --raw-output '.vers')
		echo "vers: $vers"
		if [ "$vers" = "$RELEASE_TAG" ]; then
			echo 'found'
			exit 0
		fi
	done < <(curl -fsSL -H 'Cache-Control: no-cache' "$url")

	wait_for_version "$1"
)

publish() (
	echo "::group::Publishing $1@$RELEASE_TAG..."
	cd ./crates/"$1"
	cargo publish
	echo '::endgroup::'
	echo "::group::Waiting for $1@$RELEASE_TAG to be available..."
	wait_for_version "$1"
	echo '::endgroup::'
)

publish pretty-exec-lib
publish pretty-exec
