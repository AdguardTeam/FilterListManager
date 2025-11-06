# AdGuard Filter Directives Expansion Problem

This example demonstrates a critical issue that can occur when using differential updates with filters containing AdGuard directives (`!#include`, `!#if`/`!#else`/`!#endif`, etc.).

## Problem Description

The problem can arise if your application automatically expands AdGuard directives when loading filters:
- `!#include ./additional_rules.txt` directives are replaced with the content of `additional_rules.txt` file
- Conditional directives `!#if (condition)`/`!#else`/`!#endif` are processed based on conditions
- Other directives may also be expanded

**Example of expansion:**
```
!#include ./additional_rules.txt  →  ||ads.included.com^
                                     ||trackers.included.net^
                                     ||social.included.org^$third-party
```

The problem occurs when an application:
1. Loads a filter and expands all directives
2. Applies a differential patch to the already expanded content
3. Gets an incorrect result because the patch was created for the non-expanded version

## Example Files

### Original filter versions (as stored on server):
* [filter_v1.0.0.txt](./filter_v1.0.0.txt) - base version with directives
* [filter_v1.0.1.txt](./filter_v1.0.1.txt) - intermediate version
* [filter.txt](./filter.txt) - final version
* [additional_rules.txt](./additional_rules.txt) - file included via `!#include` directive

### Expanded versions (as they look after application processing):
* [filter_v1.0.0_expanded.txt](./filter_v1.0.0_expanded.txt) - base version after directive expansion
* [filter_v1.0.1_expanded.txt](./filter_v1.0.1_expanded.txt) - intermediate version after expansion
* [filter_v1.0.2_expanded.txt](./filter_v1.0.2_expanded.txt) - final version after expansion

### Patches:
* [patches/v1.0.0-m-28334060-60.patch](./patches/v1.0.0-m-28334060-60.patch) - patch from v1.0.0 to v1.0.1
* [patches/v1.0.1-m-28334120-60.patch](./patches/v1.0.1-m-28334120-60.patch) - patch from v1.0.1 to v1.0.2 (final version)
* [patches/v1.0.2-m-28334180-60.patch](./patches/v1.0.2-m-28334180-60.patch) - empty patch

## Problem Demonstration

### Correct application (on server):
```bash
# Applying patch to non-expanded version works correctly
patch filter_v1.0.0.txt < patches/v1.0.0-m-28334060-60.patch
# Result matches filter_v1.0.1.txt
```

### Incorrect application (in application with directive expansion):
```bash
# If application expanded directives, applying patch to expanded version gives wrong result
patch filter_v1.0.0_expanded.txt < patches/v1.0.0-m-28334060-60.patch
# Result will NOT match filter_v1.0.1_expanded.txt
```

**Detailed problem scenario:**

1. Application loads `filter_v1.0.0.txt` (contains `!#include` and `!#if` directives)
2. Application expands directives → gets `filter_v1.0.0_expanded.txt` (16 lines)
3. Application applies patch `v1.0.0-m-28334060-60.patch` to expanded version
4. Patch was created for non-expanded version (15 lines), but applied to expanded (16 lines)
5. **RESULT: incorrect patch application and filter corruption, or checksum validation failure if checksums are used**

**Correct process:**
1. Always keep original non-expanded versions for patching
2. Apply patches to non-expanded versions
3. Expand directives separately when needed for filtering

## Recommendations for Developers

1. Always store and use original (non-expanded) versions for patching
2. Keep directive expansion separate from the patching process
3. Or use separate update mechanisms for filters with directives

## How Patches Were Created

```bash
# Creating first patch with validation
FILENAME="filter_v1.0.1.txt" && \
PATCHFILE="patches/v1.0.0-m-28334060-60.patch" && \
diff -n filter_v1.0.0.txt filter_v1.0.1.txt > $PATCHFILE && \
SHASUM=$(shasum -a 1 $FILENAME | awk '{print $1}') && \
    NUMLINES=$(wc -l < $PATCHFILE | awk '{print $1}') && \
    echo "diff checksum:$SHASUM lines:$NUMLINES" | cat - $PATCHFILE > temp.patch && \
    mv temp.patch $PATCHFILE

# Creating second patch with validation
FILENAME="filter.txt" && \
PATCHFILE="patches/v1.0.1-m-28334120-60.patch" && \
diff -n filter_v1.0.1.txt filter.txt > $PATCHFILE && \
SHASUM=$(shasum -a 1 $FILENAME | awk '{print $1}') && \
    NUMLINES=$(wc -l < $PATCHFILE | awk '{print $1}') && \
    echo "diff checksum:$SHASUM lines:$NUMLINES" | cat - $PATCHFILE > temp.patch && \
    mv temp.patch $PATCHFILE

# Creating empty patch for final version
touch patches/v1.0.2-m-28334180-60.patch
``` 