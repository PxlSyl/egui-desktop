# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.4] - 2026-03-05

### Added

- **Responsive menu bar**: Menu items automatically adapt to available width; items that do not fit are moved into an overflow area
- **Overflow menu (dots / hamburger)**: When space is limited, a "…" (dots) indicator shows overflow items in a dropdown; in minimal mode (no items fit), a hamburger icon opens the full menu list (hamburger available in **static** or **animated** style)
- **Full keyboard navigation**: Activate with Alt or Ctrl+F2; arrow keys move selection, Enter/Space open submenus or run actions, Left/Escape close levels; navigation works in the bar, in the overflow overlay, and in nested submenus
- **Recursive submenus to arbitrary depth**: Submenus can nest without a fixed limit; state, navigation, and rendering are fully recursive (path-based rendering with `path_prefix`, single `while` loop over `menus_to_render`)
- **Mouse/keyboard state sync**: Any mouse action (click on bar item, overflow, overlay item, submenu item, open cascade, click outside) updates keyboard state so the next keypress matches what is on screen; after validating an item with Enter in the overflow overlay, dots remain selected so Enter reopens the overlay

### Changed

- **Menu module layout**: `menu/api` renamed to `menu/core`; `rendering` and `submenu` moved under `menu/core/render/`; submenu logic split into `submenu.rs` (orchestration) and `submenu_overlay.rs` (single-level overlay drawing)
- **Overlay navigation**: Overflow overlay keyboard handling moved to `handle_overflow.rs` (`handle_overflow_overlay`); same logic for both dots and pure hamburger mode
- **Recursive rendering**: Cascade overlays are rendered in a single growing list with `path_prefix` per overlay; keyboard filter and selection use depth from path length

## [0.2.3] - 2026-02-26

### Changed

- **README refactoring**: Major restructuring and cleanup of documentation

### Fixed

- **Multi-window example**: Fixed shift layout issue where secondary windows appeared briefly at wrong position before centering
- **Module organization**: Completed refactoring of `src/menu/api.rs` into modular structure with separate files for better maintainability
  - **Import cleanup**: Removed unused imports and optimized module dependencies

## [0.2.2] - 2026-02-01

### Changed

- **macOS screenshots**: Updated README screenshots to reflect latest UI improvements

### Fixed

- **macOS traffic lights**: Improved button glyph coloring for better visibility
- **macOS cluster hover**: Implemented proper hover behavior for traffic light button clusters

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

[0.2.4]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.4
[0.2.3]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.3
[0.2.2]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.2
[0.2.1]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.1
[0.2.0]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.2.0
[0.1.9]: https://github.com/PxlSyl/egui-desktop/releases/tag/v0.1.9
