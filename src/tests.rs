use std::{path::PathBuf, fs::{self, File}, io::Write};

pub struct EmptyDb;

// if size is more than 20, valid video file content will be filled to given path
pub fn create_file(path: PathBuf, size: usize) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();

    let mut f = File::create(&path).unwrap();
    let a = fs::read("tests/resources/video.mp4").unwrap();
    f.write_all(&a[..size]).unwrap();
}