# AGENTS.md

## Project Overview

AdGuard Filter List Manager (FLM) is a Rust library for managing AdGuard filter
lists. It is used by different AdGuard applications (desktop, mobile) to integrate filter registries
([Filters Registry](https://github.com/AdguardTeam/FiltersRegistry),
[Hostlists Registry](https://github.com/AdguardTeam/HostlistsRegistry)),
check for updates, download them, implement differential updates, and more.

The library stores filter data in a local SQLite database and exposes its
functionality through a facade trait `FilterListManager`.

**This is a public open-source repository.** Never hardcode secrets, API keys,
or any sensitive credentials in the source code. Cryptographic constants like
derivation contexts and salts are fine — they are public protocol parameters,
not secrets.

## Repository Structure

```
crates/
├── filter-list-manager/   # Core library (crate: adguard-flm)
│   ├── src/
│   │   ├── lib.rs         # Public API re-exports
│   │   ├── manager/       # Main facade, models, update logic
│   │   ├── storage/       # SQLite DB layer: schema, migrations, repositories
│   │   ├── filters/       # Filter parsing, metadata extraction, preprocessor directives
│   │   ├── io/            # HTTP client, file I/O
│   │   └── utils/         # Shared utilities
│   └── resources/
│       └── sql/
│           ├── schema.sql          # Initial DB schema
│           └── migrations/         # Numbered SQL migration files (NNN-migration.sql)
├── ffi/                   # FFI wrapper (crate: adguard-flm-ffi)
│   ├── src/
│   │   ├── lib.rs                  # Thread-safe RwLock wrapper around FilterListManagerImpl
│   │   ├── native_interface/       # C ABI exports
│   │   ├── protobuf/               # .proto definitions for cross-language serialization
│   │   ├── protobuf_generated/     # Generated Rust code from .proto + conversion casts
│   │   └── platforms/              # Platform-specific builds (android/, apple/, windows/)
│   └── tests/
│       └── integration_test.rs
├── cli/                   # CLI tool (crate: adguard-flm-cli)
└── ffi-native-assets-generator/    # Helper for generating native assets
```

## Architecture

### Core Library (`adguard-flm`)

- **Facade pattern**: `FilterListManager` trait (`manager/mod.rs`) defines the
  public API. `FilterListManagerImpl` (`manager/filter_list_manager_impl.rs`)
  is the sole implementation.
- **Configuration**: `Configuration` struct
  (`manager/models/configuration/mod.rs`) holds all settings — DB path,
  locale, metadata URLs, proxy, expiration defaults, compilation policy, etc.
- **Filter types**: Index filters (from registry), Custom filters (user-added),
  Special filters (preconfigured by scripts). IDs for custom filters are
  negative (see `storage/constants.rs`).
- **Storage layer**: SQLite via `rusqlite`. Connection management through
  `DbConnectionManager`. Transactional helpers in `storage/mod.rs`.
- **Migrations**: Numbered SQL files in `resources/sql/migrations/`
  (`NNN-migration.sql`). Applied automatically by `storage/migrations.rs`.
  Schema version is tracked in the `metadata` table.
- **Filter parsing**: Metadata tag extraction (`! Title`, `! Expires`,
  `! Diff-Path`, etc.), preprocessor directives (`!#include`, `!#if/!#endif`),
  checksum validation — all in `filters/`.
- **HTTP**: `reqwest` with tokio runtime, supports proxy modes, gzip/deflate.
- **Error handling**: `FLMError` enum, `FLMResult<T>` type alias used
  throughout.

### FFI Layer (`adguard-flm-ffi`)

- Wraps `FilterListManagerImpl` in `RwLock` for thread safety.
- Uses **Protocol Buffers** (`prost`) for cross-language data serialization.
- `.proto` files are in `ffi/src/protobuf/`, generated code in
  `ffi/src/protobuf_generated/`.
- Conversion logic between FLM types and protobuf types is in
  `protobuf_generated/casts.rs`.
- Builds as `cdylib` + `staticlib` + `rlib`.
- Platform-specific wrappers: Android (Kotlin), Apple (Swift/ObjC), Windows
  (C#).

## Build & Development

- **Rust version**: 1.85+ (pinned in `rust-toolchain.toml`)
- **Workspace**: Cargo workspace with resolver v2
- **Key dependencies**: tokio, reqwest, rusqlite, serde, chrono, nom, prost
- **Features**:
  - `rusqlite-bundled` — bundles SQLite (useful for cross-compilation and tests)
  - `rustls-tls` — use rustls instead of native TLS

### Commands

| Action              | Command                                     |
|---------------------|---------------------------------------------|
| Build               | `cargo build --locked`                      |
| Run tests           | `cargo test --lib --locked`                 |
| Check formatting    | `cargo fmt --all -- --check`                |
| Lint                | `cargo clippy --locked`                     |
| Lint docs           | `npx markdownlint-cli .`                    |

### Clippy Configuration

- `allow-unwrap-in-tests = true`
- `too-many-arguments-threshold = 10`

## Versioning

- Crates `adguard-flm` and `adguard-flm-ffi` are versioned separately.
- Tags: `flm-${version}` for core, `ffi-${version}` for FFI.
- CI auto-increments patch versions on merge to master.
- See `CONTRIBUTING.md` for details.

## Critical Rules for Agents

### Database Schema Changes

- **NEVER** modify `resources/sql/schema.sql` to change the DB structure for
  existing databases. Schema changes for existing databases **MUST** go through
  a new numbered migration file in `resources/sql/migrations/`.
- Migration files follow the pattern `NNN-migration.sql` where NNN is a zerofill
  sequential number. The `metadata.schema_version` is bumped automatically
  when migrations are applied.
- Any work with storage/IO must happen only after migrations are applied.
  Treat migration execution as a hard precondition for repository access.
- Do **NOT** add runtime fallback branches for missing columns/tables
  (e.g. handling `no such column`/`no such table`) caused by unapplied
  migrations — this state should be fixed by running migrations, not masked.
- Migrations **must never modify the body** of filter rules (`rules_list.text`,
  `rules_list.disabled_text`, `filter_includes.body`) — this would silently
  invalidate integrity signatures.

### FFI & Protobuf

- Do **NOT** modify files in `ffi/src/protobuf_generated/` without
  understanding how they are generated and how the casts work.
- Any change to the public API of `FilterListManager` trait must be reflected
  in the FFI wrapper (`ffi/src/lib.rs`), the protobuf definitions
  (`ffi/src/protobuf/`), and the platform-specific bindings (android, apple,
  windows).
- Protobuf field numbers must never be reused or changed — only append new
  fields.

### Integrity Checks

Integrity protection covers **three layers** when `Configuration.integrity_key`
is set:

1. **Filter rules** — blake3 keyed hash signatures in the
   `integrity_signature` column of `rules_list` and `filter_includes` tables.
2. **Filter metadata** — blake3 keyed hash over 10 critical fields
   (`filter_id`, `download_url`, `subscription_url`, `is_trusted`,
   `is_enabled`, `is_installed`, `version`, `last_update_time`,
   `last_download_time`, `expires`) stored in `filter.integrity_signature`.
3. **Filter count** — blake3 keyed hash of the total number of filter rows,
   stored in `metadata.filter_count_signature`. Detects unauthorized addition
   or removal of filters.

#### Key rules

- Signing/verification is controlled by `Configuration.integrity_key`
  (`Option<String>`). When `None`, all integrity checks are skipped.
- The key derivation context string (`KEY_DERIVATION_CONTEXT` in
  `utils/integrity.rs`) **must never be changed** — doing so invalidates all
  existing signatures across all databases. If the signing scheme needs to
  change, bump the version suffix (e.g. `v1` → `v2`) and handle migration.
- Low-level crypto primitives live in `utils/integrity.rs`; entity-level and
  configuration-aware helpers live in
  `manager/managers/integrity_control_manager.rs`.
- `sign_all_data()` facade method **requires** `integrity_key` to be set,
  otherwise returns `InvalidConfiguration`.
- When `integrity_key` is set, the app **must** call `sign_all_data()`
  immediately after creating the FLM instance and before any other operations.
  Unsigned records will fail verification.
- To rotate the key: create a new instance with the new key, call
  `sign_all_data_with_new_key(new_key)` to re-sign all data, then proceed
  normally.

#### Verification invariants (must be maintained at all times)

Every facade method that reads or writes filter data **must** perform a
lightweight **filter count check** at the start — one metadata read, one
`COUNT(*)`, one blake3 hash.  This is implemented via
`FilterListManagerImpl::verify_filter_count_in_conn` (called inside
`execute_db` for methods whose managers accept `&Connection`) or
`FilterListManagerImpl::verify_filter_count_if_needed` (opens its own
connection, for methods whose managers still own `execute_db`).

The **only** methods exempt from this check are:

- The constructor (`new`).
- `lift_up_database` / `get_database_version` / `get_database_path`.
- `sign_all_data` / `sign_all_data_with_new_key` / `verify_integrity`.
- `fetch_filter_list_metadata` / `fetch_filter_list_metadata_with_body`
  (network-only, no DB filter access).
- `get_all_tags` / `get_all_groups` (read unprotected tables).
- `change_locale` / `set_proxy_mode` (configuration-only).

Per-entity **metadata and rules signatures** are verified on the entities that
are actually returned to the caller (not a full DB scan on every call).

A full streaming verification of **all** signatures (rules + includes +
metadata + count) is available via the public `verify_integrity()` method.

#### Signing invariants (must be maintained at all times)

Any facade method that **writes** filter data must ensure that all affected
integrity signatures are updated within the same transaction:

- When a filter row is created, updated, enabled, installed, or deleted: its
  **metadata signature** and the **count signature** in `metadata` must be
  re-signed.
- When filter rules or includes are written: their **rules/includes
  signatures** must be re-signed.
- New facade methods or code paths that modify protected tables (`filter`,
  `rules_list`, `filter_includes`) **must** follow these patterns — otherwise
  subsequent reads will fail with `FilterIntegrityCheckFailed`.

Do **NOT** run `cargo clippy` yourself — the maintainer is aware of warnings
and will fix them separately.

### Filter ID Ranges

- Index filter IDs are positive integers from the registry.
- Custom filter IDs are in range `[-1_000_000_000, -10_000]`.
- Special filter IDs (e.g., `USER_RULES_FILTER_LIST_ID`) use `i32::MIN`.
- `SMALLEST_POSSIBLE_FILTER_ID` (`-2_000_000_000`) is reserved — library will
  never create a filter with this ID or lower.

### Tests

- Do **NOT** delete or weaken existing tests.
- `unwrap()` is allowed in test code (configured in `clippy.toml`).
- Tests create temporary SQLite databases (`agflm_*.db`) that are gitignored.

### Code Style

- Follow existing patterns: facade trait + impl, repository pattern for DB
  access, `FLMResult<T>` for error handling.
- All public types are re-exported from `lib.rs`.
- Use `thiserror` for error types.
- SQL in Rust code must follow the repository style used in
  `crates/filter-list-manager/src/storage/repositories/filter_repository.rs`:
  SQL operators (`SELECT`, `FROM`, `WHERE`, `UPDATE`, `SET`, `INSERT INTO`)
  start on a new line, table names are wrapped in square brackets
  (for example `[filter]`), and selected/updated column names are listed one
  per line. For `INSERT`, keep the closing parenthesis of the column list and
  `VALUES` on the same line in the form `) VALUES (`.
- Modules marked `#[doc(hidden)]` or `pub(crate)` are internal — avoid
  expanding their visibility without good reason.
- Do NOT use `unwrap()`, `expect()`, `panic!()` or any other constructs that
  can crash the application in production code. Handle errors explicitly and
  return `FLMResult<T>` with proper `FLMError`. The ONLY exception is in
  `ffi/src/native_interface/mod.rs`, where some low-level FFI boundaries may
  require hard failures. Note: `unwrap()` is allowed in test code (see Tests
  section).

### Platform Builds

- The FFI crate targets Android (arm64, armv7, x86, x86_64), Apple (iOS,
  macOS, both architectures), and Windows.
- Do not modify platform-specific build scripts without understanding
  cross-compilation requirements.
