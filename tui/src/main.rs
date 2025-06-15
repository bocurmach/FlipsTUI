mod find_patches;
use find_patches::find_patches;
mod patch_files;
use patch_files::patch_files;
use std::env;

/*
* Order of procedure:
* (1) selecting one or multiple patches and/or zip files with patches from a designated folder (i.e. Downloads)
* (2) patching a default file or any other selected file
*    (i) The patching will done by the original C code
* (3) move the patched files to a default or selected folder
* (4) deleting of used patches/zip files with patches or archiving them to a designated folder
* (5) make a backup of a folder i.e. saves on the sd card if selected or by default
*/

/*
* WHAT THIS PROGRAM DOES FROM A USER POV
* (FO = Feature Option)
* (NA = Necessary Action the user has to take)
* 1. (NA) Open up TUI
* 2. I am being shown all .bps files in the default (Download) path and all .bps files in the zip
* files which are distinguished
*  - (FO) There is an option, to select a different path
* 3. (NA) I can select .bps files in that view/list and mark them for patching and deletion/archiving of
*    the .bps and/or .zip files they are in
*  - Make all of them selectable or none
*  - It has to follow a default rule (config file) regarding of what happens with the .bps/.zip
*    files. Selecting that is inconvenient.
*  - It has to be adaptable, to archive some, delete some other and do nothing with the rest.
*  - (FO) Make it possible to save the default to the config file for next time.
* 4. (NA) I confirm the selection of patches and what to do with them afterward
* 5. I can select a .smc file to patch. Default in config
*    - (FO) Show a list of .smc files that have been specified in config, with one being the
*       default of those.
* 6. (NA) I confirm the .smc file
* 7. I can select a dir (existent or non existent) where those patched files are going to be saved
*    in.
*    - (FO) Different dirs for different files
* 8. (NA) I confirm the output dir
* 9. Confirmation it worked or list of errors into stdout after TUI exited
*
* NOTE: to reduce enter presses (NA) for 3., 4., 6., 8. it would be interesting to have those steps
* all on one screen and combine those enter pressess/confirmations to go on into one START command.
*/

fn main() {
    // (1) Selecting patches
    let patches = find_patches(&format!("{}/Downloads", env::var("HOME").unwrap()));
    let patches = patches.unwrap();

    for patch in &patches {
        println!("{:?}", patch)
    }

    // (2) patching a file (or pick one)
    patch_files(&patches)
}
