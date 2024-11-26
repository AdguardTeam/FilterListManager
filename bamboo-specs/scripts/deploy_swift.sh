#!/bin/bash

set -e
set -x

# Check that we are on the master branch
if [ "${bamboo_repository_branch_name}" != "master" ] && [ "${bamboo_repository_branch_name}" != "0.8.x" ]; then
  echo "Not on the master branch. Exiting..."
  exit 0
fi

ARTIFACTORY_USER="${bamboo_artifactoryUser}"
ARTIFACTORY_PASS="${bamboo_artifactoryPassword}"
ARTIFACTORY_PATH="https://${bamboo_artifactoryHostname}/artifactory/adguard-pods/binaries/AdGuardFLM"

SUFFIX="" # Always release build
VER="$(sed -ne 's/^ *version = \"\(.*\)\"/\1/p' crates/ffi/Cargo.toml)"
VER="${VER}${SUFFIX}"

ARCH_NAME="AdGuardFLM-${VER}.zip"

PODSPEC=$(cat << EOF
{
  "name": "AdGuardFLM",
  "version": "${VER}",
  "summary": "AdGuard FiltersListManager",
  "description": "AdGuard FiltersListManager library",
  "homepage": "https://${bamboo_bitbucketHostname}/projects/ADGUARD-CORE-LIBS",
  "documentation_url": "https://${bamboo_bitbucketHostname}/projects/ADGUARD-CORE-LIBS",
  "screenshots": "https://${bamboo_bitbucketHostname}/projects/ADGUARD-CORE-LIBS/avatar.png",
  "license": {
    "type": "proprietary",
    "file": "LICENSE"
  },
  "authors": {
    "Adguard Software Ltd": "devteam@adguard.com"
  },
  "platforms": {
    "osx": "10.15",
    "ios": "11.2"
  },
  "source": {
    "http": "${ARTIFACTORY_PATH}/${ARCH_NAME}"
  },
  "preserve_paths": ["AdGuardFLM.xcframework"],
  "vendored_frameworks": "AdGuardFLM.xcframework",
  "xcconfig": {
    "LD_RUNPATH_SEARCH_PATHS": "@loader_path/../Frameworks"
  },
  "requires_arc": true
}
EOF
)

cd platform/apple/build/framework

echo "#${VER}" > CHANGELOG
echo "Confidential. Property of Adguard Software Ltd. https://adguard.com" > LICENSE
zip -4yr "${ARCH_NAME}" AdGuardFLM.xcframework CHANGELOG LICENSE
curl -u"${ARTIFACTORY_USER}:${ARTIFACTORY_PASS}" -XPUT "${ARTIFACTORY_PATH}/${ARCH_NAME}" -T "${ARCH_NAME}"

git checkout --detach && git branch -D swiftpm || true
git checkout --orphan swiftpm
git reset

SPM_TAG="v${VER}@swift-5"
: > Package.swift # Crutch for `swift package compute-checksum`
echo '// swift-tools-version:5.3
import PackageDescription

let package = Package(
  name: "AdGuardFLM",
  platforms: [
    .iOS("11.2"), .macOS("10.15")
  ],
  products: [
    .library(name: "AdGuardFLM", targets: ["AdGuardFLM"]),
  ],
  targets: [
    .binaryTarget(
      name: "AdGuardFLM",
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/'${SPM_TAG}/${ARCH_NAME}'",
      checksum: "'$(swift package compute-checksum ${ARCH_NAME})'"
    ),
  ]
)
' > ../../../../Package.swift

git add ../../../../Package.swift
git commit -m "AdGuardFLM for SwiftPM ${VER}"
git tag -d "${SPM_TAG}" || true
git tag "${SPM_TAG}"
git remote set-url origin "${bamboo_planRepository_1_repositoryUrl}"
git push origin "${SPM_TAG}"
git remote add gh https://${bamboo_githubPublicRepoPassword}:@github.com/AdguardTeam/FilterListManager/
git push gh "${SPM_TAG}" || true
gh config set -h github.com oauth_token "${bamboo_githubPublicRepoPassword}" || exit 1
gh release create ${SPM_TAG} -t "v${VER} for SwiftPM" -n "Prebuilt package for SwiftPM"
gh release upload ${SPM_TAG} ${ARCH_NAME}

rm "${ARCH_NAME}"

cd ../../../../podspecs
git reset
git checkout master
git remote set-url origin "${bamboo_planRepository_2_repositoryUrl}"
git pull

SPEC_DIR="Specs/AdGuardFLM/${VER}"
mkdir -p "${SPEC_DIR}"
cd "${SPEC_DIR}"

echo -n "${PODSPEC}" > AdGuardFLM.podspec.json
git add AdGuardFLM.podspec.json
git commit -m "AdGuardFLM ${VER}" || true
git push origin HEAD
