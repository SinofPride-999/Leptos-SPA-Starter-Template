<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos SPA Template

A modern, production-ready Single Page Application (SPA) template built with [Leptos](https://leptos.dev/), Rust, and WebAssembly.

## Why This Template Was Built

When starting a new Leptos SPA project, developers often face challenges:

- **No Official SPA Template**: Leptos primarily focuses on SSR (Server-Side Rendering), and there's no easily installable SPA template available out of the box.
- **Complex Setup**: Setting up a Leptos SPA requires understanding WASM compilation, Trunk configuration, and project structure.
- **Missing Best Practices**: Many starter projects lack proper architectural patterns for scalability.

This template was created to solve these problems by providing a **ready-to-use, production-ready SPA foundation** that follows modern web development best practices.

## Purpose

This template serves as a **starting point** for developers who want to build:

- Single Page Applications with Leptos
- High-performance web apps using WebAssembly
- Projects requiring modular, scalable architecture
- Applications that benefit from Rust's memory safety and performance

## Features

- 📦 **Modular Architecture**: Pre-structured folders for components, pages, routes, state, and services
- 🛣️ **Routing Ready**: Client-side routing pre-configured with `leptos_router`
- 🎨 **dTailwind CSS Pre-configure**: Easily style components using Tailwind classes
- ⚡ **Blazing Fast**: Compiles to WebAssembly for near-native performance in the browser.
- 🧩 **Reusable Components**: Pre-made example components (like Button) that demonstrate reactive patterns.
- 🏗️ **Trunk-Ready**: Built with Trunk, so bundling, live reloading, and asset management are ready to go.

## Tech Stack

- **Framework**: Leptos 0.8 (CSR mode)
- **Language**: Rust
- **Build Tool**: Trunk
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **CSS**: Custom modern styles with glass morphism effects
- **Routing**: Leptos Router

## Project Structure

```
leptos-spa-template/
├── Cargo.toml              # Rust project configuration
├── trunk.toml             # Trunk build configuration
├── index.html             # HTML entry point
├── README.md              # This file
├── assets/                # Static assets (favicon, etc.)
└── src/
    ├── main.rs           # Application entry point
    ├── app.rs            # Root App component with Router
    ├── components/       # Reusable UI components
    │   ├── mod.rs
    │   └── button.rs     # Example interactive component
    ├── pages/            # Page-level components
    │   ├── mod.rs
    │   └── home_page.rs  # Home page with features showcase
    ├── routes/           # Route definitions
    │   ├── mod.rs
    │   └── home_routes.rs
    ├── state/            # Global state management (ready for use)
    │   └── mod.rs
    ├── services/         # API/services layer (ready for use)
    │   └── api.rs
    └── utils/            # Utility functions (ready for use)
        └── mod.rs
```

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust**: Install via [rustup.rs](https://rustup.rs/)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm32-unknown-unknown target**:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (Rust WASM bundler):
   ```bash
   cargo install trunk
   ```

### Clone the Template

Clone this repository to your local machine:

```bash
# Clone the repository
git clone https://github.com/SinofPride-999/Leptos-SPA-Starter-Template

# Navigate into the project directory
cd leptos-spa-starter-template

# (Optional) Rename the project
# Edit Cargo.toml and change the name field
```

### Install Dependencies

No additional npm/yarn dependencies needed! All dependencies are managed by Cargo:

```bash
cargo fetch
```

### Run Development Server

Start the development server with live reload:

```bash
trunk serve
```

This will:

- Build the WASM bundle
- Start a local server at `http://localhost:8080`
- Automatically open your browser
- Watch for file changes and rebuild automatically

### Build for Production

Build an optimized production release:

```bash
trunk build --release
```

The compiled files will be in the `dist/` directory, ready for deployment.

## 📖 Usage Guide

### Adding New Pages

1. Create a new file in `src/pages/` (e.g., `about_page.rs`)
2. Define your page component:

   ```rust
   use leptos::prelude::*;

   #[component]
   pub fn AboutPage() -> impl IntoView {
       view! {
           <div>
               <h1>"About Us"</h1>
               // Your content here
           </div>
       }
   }
   ```

3. Export it in `src/pages/mod.rs`
4. Add the route in `src/routes/`

### Adding New Components

1. Create a new file in `src/components/`
2. Define your component
3. Export in `src/components/mod.rs`
4. Import and use in any page

### Adding New Routes

Edit `src/routes/home_routes.rs` to add more routes:

```rust
use leptos_router::{components::*, path};

use crate::pages::AboutPage;

#[component]
pub fn HomeRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=HomePage/>
            <Route path=path!("/about") view=AboutPage/>
        </Routes>
    }
}
```

### Using State Management

The template includes a `state/` folder ready for global state. Example with signals:

```rust
// src/state/mod.rs
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub count: RwSignal<i32>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            count: RwSignal::new(0),
        }
    }
}

pub fn use_app_state() -> AppState {
    AppState::default()
}
```

### Making API Calls

Use the `services/` folder for API integrations:

```rust
// src/services/api.rs
use leptos::task::spawn_local;

pub async fn fetch_data() -> Result<serde_json::Value, reqwasm::Error> {
    reqwasm::http::Request::get("https://api.example.com/data")
        .send()
        .await?
        .json()
        .await
}
```

## 🎨 Customization

### Changing the Title

Edit `index.html`:

```html
<title>Your App Name</title>
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Leptos Framework](https://leptos.dev/) for this amazing reactive framework
- [Trunk](https://github.com/thedodd/trunk) for WASM bundling
- The Rust community for continuous inspiration

---

**Happy Coding! 🚀**
