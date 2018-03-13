#!/bin/bash

# Install Rust
curl https://sh.rustup.rs -sSf | bash -s -- -y
if [ "$TRAVIS_OS_NAME" == "osx" ]; then
  brew update || brew update

  brew outdated openssl || brew upgrade openssl
  brew install openssl@1.1

  # install pyenv
  git clone --depth 1 https://github.com/pyenv/pyenv ~/.pyenv
  PYENV_ROOT="$HOME/.pyenv"
  PATH="$PYENV_ROOT/bin:$PATH"
  eval "$(pyenv init -)"

  case "${TOXENV}" in
    py27)
      curl -O https://bootstrap.pypa.io/get-pip.py
      python get-pip.py --user
      ;;
    py36)
      pyenv install 3.6.1
      pyenv global 3.6.1
      ;;
  esac
  pyenv rehash

  # A manual check that the correct version of Python is running.
  python --version
fi
