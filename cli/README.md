# egui-desktop CLI

Simple CLI tool to initialize new egui-desktop projects with a complete modular starter template.

## Installation

```bash
cargo install --path cli
```

## Usage

```bash
egui-desktop my-awesome-app
```

This will create a new directory `my-awesome-app` with a complete modular project structure:

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
