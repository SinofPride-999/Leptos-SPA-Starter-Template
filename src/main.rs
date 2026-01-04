use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;
mod routes;
// mod state;
// mod services;
// mod utils;

use app::App;


fn main() {
    // mount SPA app
    mount_to_body(|| App());
}
