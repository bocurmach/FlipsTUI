use std::fs::{File, read_dir};
use std::io;
use std::path::PathBuf;
use zip::read::ZipArchive;

fn patch_in_zip(zip_file_name: &str) -> zip::result::ZipResult<()> {
    let file = File::open(zip_file_name)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;

        if !file.is_dir() && file.name().ends_with(".bps") {
            file.name();
        }
    }
    Ok(())
}

pub fn find_patches(patches_dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut zip_patches: Vec<PathBuf> = Vec::new();

    for entry in read_dir(patches_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let fname = path.clone().into_os_string().into_string().unwrap();

            if fname.ends_with(".zip") {
                zip_patches.push(path);
                let _ = patch_in_zip(&fname);
            } else if fname.ends_with(".bps") {
                zip_patches.push(path);
            }
        }
    }
    Ok(zip_patches)
}
