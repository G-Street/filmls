use std::ffi::OsStr;
use std::path::Path;

pub fn get_extension_from_filename(filename: &Path) -> Option<&str> {
    filename.extension().and_then(OsStr::to_str)
}
