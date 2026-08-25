#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    repro_lib::run();
}
