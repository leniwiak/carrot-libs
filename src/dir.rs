#![allow(dead_code)]
use std::path::Path;
use std::path::PathBuf;
use std::fs;

pub fn browse(dir:&Path) -> Result<Vec<PathBuf>, String> {
    let mut list = Vec::new();
    let mut error = None;
    for r in fs::read_dir(dir).unwrap() {
        match r {
            Err(e) => {
                error = Some(format!("{}: Failed to list directory because of an error: {:?}", dir.display(), e.kind()));
            },
            Ok(e) => list.push(e.path()),
        };
    };
    if let Some(value) = error {
        Err(value)
    } else {
        Ok(list)
    }
}
