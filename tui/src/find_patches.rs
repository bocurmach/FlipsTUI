use std::fs;
use std::io;
use std::path::PathBuf;

pub fn find_patches(patches_dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut zip_patches: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(patches_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let fname = path.clone().into_os_string().into_string().unwrap();

            if fname.ends_with(".zip") {
                println!("{}", fname);
                zip_patches.push(path);
            }
        }
    }
    Ok(zip_patches)
}
