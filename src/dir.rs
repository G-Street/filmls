// This file provides functions used for dynamically locating the media directory
use std::path::PathBuf;

// Films and series directory names
#[derive(PartialEq)]
pub enum MediaType {
    Film,
    Series,
    Unknown,
    Impossible,
}

// https://stackoverflow.com/a/65040451
impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Film => "Films",
            MediaType::Series => "Series",
            MediaType::Unknown => "<Unknown>",
            MediaType::Impossible => "<Impossible>",
        }
    }
}

// Conditionally compiling functions for obtaining media directoried
// Source: https://doc.rust-lang.org/rust-by-example/attribute/cfg.html, https://doc.rust-lang.org/reference/conditional-compilation.html#target_os

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
pub fn get_media_dir() -> PathBuf {
    std::path::PathBuf::from("/mnt/Primary/Media/")
}

#[cfg(target_os = "macos")]
pub fn get_media_dir() -> PathBuf {
    std::path::PathBuf::from("/Volumes/Media/")
}

// Fallback on current dir
#[cfg(not(any(target_os = "freebsd", target_os = "linux", target_os = "macos")))]
pub fn get_media_dir() -> PathBuf {
    std::env::current_dir().expect("Cannot get current directory")
}
