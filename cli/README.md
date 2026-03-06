# egui-desktop CLI

Simple CLI tool to initialize new egui-desktop projects with a complete modular starter template.

## Installation

```bash
cargo install egui-desktop-cli
```

## Usage

```bash
egui-desktop my-awesome-app
```

This will create a new directory `my-awesome-app` with a complete modular project structure.

### Testing without publishing the crate

To try the generated project against the **local** egui-desktop crate (e.g. before publishing):

- **From the egui-desktop repo root** (project created inside the repo):
  ```bash
  cargo run --manifest-path cli/Cargo.toml -- my-app --local
  cd my-app && cargo run
  ```
  This uses `egui-desktop = { path = ".." }` in the generated `Cargo.toml`.

- **From anywhere** with an explicit path to the crate:
  ```bash
  egui-desktop my-app --path /path/to/egui-desktop-ui
  cd my-app && cargo run
  ```

Alternatively, after generating a project, you can edit its `Cargo.toml` and replace  
`egui-desktop = "0.2.4"` with `egui-desktop = { path = "../egui-desktop-ui" }` (adjust the path as needed).

```
my-awesome-app/
├── Cargo.toml
└── src/
    ├── main.rs              # Application entry point
    ├── lib.rs               # Module declarations and exports
    ├── app.rs               # Main application struct and logic
    ├── theme_provider.rs    # Custom theme system (Ocean, Forest)
    ├── sidebar.rs           # Sidebar and main content rendering
    └── icons.rs             # Custom icon drawing functions
```

## What it does

- Creates a new project directory with proper structure
- Generates a modular codebase (6 files instead of 1 monolithic file)
- Copies the complete starter template with all features
- Generates a `Cargo.toml` with the correct dependencies
- Ready to run with `cargo run`

## Example

```bash
egui-desktop my-app
cd my-app
cargo run
```

Your app will run with all the features from the starter:

### ✨ Features Included

- **🎨 Custom Themes**: Ocean (blue) and Forest (green) themes in light/dark variants
- **⌨️ Keyboard Navigation**: Full Alt+Arrow keys menu navigation
- **📋 Complex Menus**: Multi-level menus with submenus and shortcuts
- **🎛️ Interactive Sidebar**: Theme controls and live preview
- **🪟 Window Features**: Native rounded corners and resize handles
- **🎯 Professional Structure**: Modular code organization showing best practices

### 🏗️ Project Structure Benefits

- **Modular Design**: Each component in its own file for better maintainability
- **Clean Architecture**: Separation of concerns (UI, themes, logic)
- **Learning Resource**: Shows how to organize egui applications properly
- **Extensible**: Easy to add new features or modify existing ones

## Development

The CLI uses a modular starter template located in `cli/src/starter/` that demonstrates:

- How to structure egui applications
- Custom theme implementation
- Menu system with keyboard shortcuts
- Sidebar animations and interactions
- Cross-platform window features

This makes it an excellent starting point for both learning and production applications.
