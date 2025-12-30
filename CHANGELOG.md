# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-12-XX

### Changed

- **Migration to objc2**: Updated macOS dependencies from `objc`/`cocoa` to `objc2`/`objc2-app-kit`/`objc2-foundation` for better maintainability and type safety
- Improved macOS rounded corners implementation with better background transparency handling
- **CLI version**: Updated `egui-desktop-cli` to version 0.2.1 to match the main crate

### Fixed

- Fixed submenu positioning offset on macOS (submenus now correctly align below the thinner 28px title bar)
- Fixed gray background appearing in rounded corners on macOS windows

## [0.2.0] - 2025-09-XX

### Added

- **Multi-window support**: Complete example demonstrating multiple native windows (viewports) with independent `TitleBar` instances
- **Rounded corners for secondary windows**: `apply_rounded_corners_to_viewport()` function to apply native rounded corners to secondary viewports
- **Window centering**: Secondary windows are automatically centered over the primary viewport
- **Example**: New `multi_window.rs` example showing how to create and manage multiple windows
- **Platform-specific window handle retrieval**: Automatic window handle detection for secondary viewports using platform APIs (Windows, macOS)

### Changed

- Updated README with comprehensive multi-window documentation
- Improved rounded corners implementation to handle window reopening
- **CLI version**: Updated `egui-desktop-cli` to version 0.2.0 to match the main crate

### Fixed

- Fixed rounded corners not being reapplied when secondary windows are closed and reopened
- Fixed `opt-level` in Cargo.toml (was string, now integer)

## [0.1.9] - Previous version

Initial alpha release with basic title bar, menu system, and theming support.

[0.2.1]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.1
[0.2.0]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.0
[0.1.9]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.1.9
