#!/bin/bash

set -e

increment_version() {
  major=${1%%.*}
  minor=$(echo ${1#*.} | sed -e "s/\.[0-9]*//")
  revision=${1##*.}
  echo ${major}.${minor}.$((revision+1))
}

increment_flm_version() {
  if [ -z "$1" ]; then
    # Increment the version of the `filter-list-manager` crate
    echo "No custom version provided. Incrementing revision..."
    CURRENT_VERSION=$(sed -ne 's/^ *version = \"\(.*\)\"/\1/p' crates/filter-list-manager/Cargo.toml)
    NEW_VERSION=$(increment_version $CURRENT_VERSION)
    echo "Current version: $CURRENT_VERSION"
  else
    echo "Custom version provided: $1. Using it..."
    NEW_VERSION=$1
  fi
  echo "New version: $NEW_VERSION"
  sed -i "s/^ *version = \".*\"/version = \"$NEW_VERSION\"/" crates/filter-list-manager/Cargo.toml
  echo "Version incremented successfully!"
}

increment_flm_ffi_version() {
  if [ -z "$1" ]; then
    # Increment the version of the `ffi` crate
    echo "No custom version provided. Incrementing revision..."
    CURRENT_VERSION=$(sed -ne 's/^ *version = \"\(.*\)\"/\1/p' crates/ffi/Cargo.toml)
    NEW_VERSION=$(increment_version $CURRENT_VERSION)
    echo "Current version: $CURRENT_VERSION"
  else
    echo "Custom version provided: $1. Using it..."
    NEW_VERSION=$1
  fi
  echo "New version: $NEW_VERSION"
  sed -i "s/^ *version = \".*\"/version = \"$NEW_VERSION\"/" crates/ffi/Cargo.toml
  echo "Version incremented successfully!"
}

# Check that we are on the master branch
if [ "${bamboo_repository_branch_name}" != "master" ] && [ "${bamboo_repository_branch_name}" != "0.8.x" ]; then
  echo "Not on the master branch. Exiting..."
  exit 0
fi

if [ ${bamboo_adguard_flm_custom_version} = "none" ]; then
  bamboo_adguard_flm_custom_version=
fi

if [ ${bamboo_ffi_custom_version} = "none" ]; then
  bamboo_ffi_custom_version=
fi

# Check if there are any changes in the `filter-list-manager` crate
HASH_FILE="platform/flm_version_hash.hash"
OBSERVED_DIR="crates/filter-list-manager"

# Get the hash of the current state of the `filter-list-manager` crate
CURRENT_HASH=$(find $OBSERVED_DIR -type f -exec md5sum {} \; | md5sum | awk '{print $1}')

# Get the hash of the last state of the `filter-list-manager` crate
PREVIOUS_HASH=$(cat $HASH_FILE)

# If the hash has changed, increment the version
if [ "$CURRENT_HASH" != "$PREVIOUS_HASH" ]; then
  echo "The hash has changed. Incrementing the 'filter-list-manager' version..."
  increment_flm_version ${bamboo_adguard_flm_custom_version}
  # Update the hash file after the version increment
  CURRENT_HASH=$(find $OBSERVED_DIR -type f -exec md5sum {} \; | md5sum | awk '{print $1}')
  echo $CURRENT_HASH > $HASH_FILE
else
  echo "The hash has not changed. Skipping the 'filter-list-manager' version increment..."
fi

# Increment the version of the `ffi` crate
echo "Incrementing the 'ffi' version..."
increment_flm_ffi_version ${bamboo_ffi_custom_version}

# Configure git
git config user.name "Bamboo"
git config user.email "Bamboo"

# Update the remote repository
./git-scripts/git_kit.sh pull ${bamboo_planRepository_repositoryUrl}
git reset

# Add the updated Cargo.toml files to the git index
git add crates/filter-list-manager/Cargo.toml
git add crates/ffi/Cargo.toml
git add $HASH_FILE

# Commit the changes
git commit -m "skipci: Automatic version increment" || exit 1
git push
