mod find_patches;
use find_patches::find_patches;
use std::env;

fn main() {
    let _ = find_patches(&format!("{}/Downloads", env::var("HOME").unwrap()));
}
