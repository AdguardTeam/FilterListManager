# Installation

To build `filter_list_manager` for Apple platforms, you should install
[Rust 1.85](https://www.rust-lang.org/tools/install)
 
# Build and development

### Available scripts

`Scripts/configure` - configure rust before first build\
`Scripts/generate_proto` - regenerate Swift-protobuf counterpart\
`Script/build` - build .xcframework for Apple OSs\
`Script/archive_framework` - archive framework, puts it to build folder

### Development
After changing rust interface you should:
1. Regenerate protobuf
2. Build xcframework
3. Run `archive_framework` script

# Test your build

1. Run **Build** steps 
2. Open `platform/apple/AdguardFLM` as local package in XCode
3. Make `Clean Build Folder` in XCode
4. Run AdGuardFLMLibTests
5. Make sure that you have tested your changeset properly
