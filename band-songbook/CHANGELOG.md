# Changelog

All notable changes to this project will be documented in this file.

## [0.0.7] - 2026-02-05

### Changed
- `make_all_with_storage` now takes `local_sandbox` as explicit parameter
- Lambda uses fixed `/tmp/sandbox` path instead of temp directory

### Improved
- lilypond-book errors now log stdout and stderr content for easier debugging

## [0.0.6] - 2026-02-04

### Fixed
- Lambda now shows application logs in CloudWatch (added `tracing-log` feature)

## [0.0.5] - 2026-02-04

### Changed
- CLI now supports S3 paths for srcdir, sandbox, and settings arguments
- Main function is now async using tokio runtime
- Uses `make_all_with_storage` for unified local/S3 path handling

## [0.0.4] - 2026-02-04

### Added
- GitHub Actions workflow for Lambda deployment (`deploy-lambda.yml`)
  - Accepts srcdir, sandbox, settings, and pattern as inputs
  - Builds with cargo-lambda for AWS Lambda
  - Deploys and optionally invokes the function

### Fixed
- Log files are now correctly uploaded to S3 (recursive directory traversal)

## [0.0.3] - 2026-02-04

### Added
- AWS Lambda function binary (`band-songbook-lambda`) for serverless builds
- Lambda accepts S3 paths for srcdir, sandbox, settings, and pattern
- S3 integration test (`test_make_all_with_s3`)

## [0.0.2] - 2026-02-04

### Added
- `get_lilypond_files` public function to extract LilyPond file dependencies from a PdfFile node
- `PdfFile` re-exported from crate root for easier access
- Pre-build check for `lualatex` availability with clear error message
- Log files are now uploaded to S3 along with PDF files
- `todo.md` for tracking future work

### Fixed
- Repository URL in Cargo.toml (now points to correct GitHub repository)

## [0.0.1] - Initial release

### Added
- Core build system for generating PDF songbooks
- Support for chord charts and LilyPond music notation
- S3 storage support for source and output files
- Song discovery and pattern-based filtering
