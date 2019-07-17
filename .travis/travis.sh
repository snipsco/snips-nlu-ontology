#!/bin/bash
set -ev

# Install Rust
if [[ -z ${TRAVIS_RUST_VERSION+w} ]]; then
  curl https://sh.rustup.rs -sSf | bash -s -- -y
fi

export PATH="$HOME/.cargo/bin:$PATH"

cargo build --all

if [[ `git status --porcelain` ]]; then
  echo "The build step produced some changes that are not versioned"
  git status
  exit 1
fi

cargo test --all

if [[ "$KOTLIN_TESTS" == "true" ]]; then
  cd platforms/kotlin
  ./gradlew -Pdebug build --info
  cd ../..
fi
