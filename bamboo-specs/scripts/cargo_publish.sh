#!/bin/bash

set -e

# Process of publishing the crates:
# Step 1: Login to the registry
# Step 2: If current version of 'adguard-flm' is already published, skip to Step 5
# Step 3: Publish 'adguard-flm' crate, dry-run first
# Step 4: Wait for the 'adguard-flm' version to be available in the registry
# Step 5: If current version of 'ffi' crate is already published, skip to Step 8
# Step 6: Replace the 'adguard-flm' version in the 'ffi' crate with the published version from previous step
# Step 7: Publish 'ffi' crate with the updated version, dry-run first
# Step 8: Logout from the registry

# Check that we are on the master branch
# shellcheck disable=SC2154
if [ "${bamboo_repository_branch_name}" != "master" ]; then
  echo "Not on the master branch. Exiting..."
  exit 0
fi

# Get the 'adguard-flm' version
ADGUARD_FLM_VERSION=$(sed -ne 's/^ *version *= *\"\(.*\)\"/\1/p' crates/filter-list-manager/Cargo.toml)
FFI_VERSION=$(sed -ne 's/^ *version *= *\"\(.*\)\"/\1/p' crates/ffi/Cargo.toml)

# Login to the registry
echo "Logging in to the registry..."
# shellcheck disable=SC2154
cargo login ${bamboo_cargoPassword}

# Check if the 'adguard-flm' version is already published
echo "Checking if the 'adguard-flm' version is already published..."
FOUND=$(cargo search adguard-flm | grep "adguard-flm = \"$ADGUARD_FLM_VERSION\"")

if [ -z "$FOUND" ]; then
  echo "The 'adguard-flm' version is not published yet!"

  # Publish the 'adguard-flm' crate
  echo "Publishing 'adguard-flm' crate..."
  pushd crates/filter-list-manager
    cargo publish --dry-run
    cargo publish
  popd

  # Wait for the 'adguard-flm' version to be available in the registry
  CRATE_NAME="adguard-flm"
  echo "Waiting for the '$CRATE_NAME' version to be available in the registry..."
  FOUND=$(cargo search $CRATE_NAME | grep "$CRATE_NAME = \"$ADGUARD_FLM_VERSION\"")
  ATTEMPT_COUNT=0
  ATTEMPT_LIMIT=20
  SLEEP_TIME=10

  while [ -z "$FOUND" ] && [ $ATTEMPT_COUNT -lt $ATTEMPT_LIMIT ]; do
    sleep $SLEEP_TIME
    FOUND=$(cargo search $CRATE_NAME | grep "$CRATE_NAME = \"$ADGUARD_FLM_VERSION\"")
    ATTEMPT_COUNT=$((ATTEMPT_COUNT+1))
  done
  if [ -z "$FOUND" ]; then
    echo "The '$CRATE_NAME' version is not available in the registry!"
    exit 1
  fi

  echo "The '$CRATE_NAME' version is available in the registry!"
fi

# Check if the 'ffi' crate is already published
echo "Checking if the 'ffi' crate is already published..."
FFI_FOUND=$(cargo search ffi | grep "ffi = \"$FFI_VERSION\"")

if [ -z "$FFI_FOUND" ]; then
  echo "The 'ffi' crate is not published yet!"
  # Replace the 'adguard-flm' version in the 'ffi' crate
  echo "Replacing the 'adguard-flm' version in the 'ffi' crate..."
  sed -i "s/^adguard-flm = .*/adguard-flm = \"$ADGUARD_FLM_VERSION\"/" crates/ffi/Cargo.toml

  # Configure git
  git config user.name "Bamboo"
  git config user.email "Bamboo"

  git add crates/ffi/Cargo.toml
  git commit

  # Publish the 'ffi' crate
  echo "Publishing 'ffi' crate..."
  pushd crates/ffi
    cargo publish --dry-run
    cargo publish
  popd
fi

cargo logout
