use std::convert::TryInto;
use std::path::Path;
use std::{fs, io};

use colored::*;

use super::constants;
use super::dir::MediaType;
use super::path;

pub fn show_count(dirname: &Path, media_type: &MediaType) {
    match media_type {
        MediaType::Film => {
            let mut films_dir = dirname.to_path_buf();
            films_dir.push(media_type.as_str());
            let cnt = count_media_files(&films_dir);
            println!(
                "{}{}{}",
                "You have ".italic(),
                cnt.to_string().bold(),
                " films in your Plex Media Server.".italic()
            );
        }
        MediaType::Series => {
            let mut series_dir = dirname.to_path_buf();
            series_dir.push(media_type.as_str());
            let mut cnt = 0;
            let series: Vec<_> = fs::read_dir(&series_dir)
                .unwrap_or_else(|_| panic!("Cannot read directory: {:?}", series_dir))
                .map(|e| e.expect("Cannot retreive file information").path())
                .collect();
            for path in series {
                if path.is_dir() {
                    let contents: Vec<_> = fs::read_dir(&path)
                        .expect("Cannot read directory")
                        .map(|e| {
                            e.expect("Cannot retreive file information")
                                .path()
                                .file_name()
                                .expect("Cannot get file name from file")
                                .to_str()
                                .unwrap()
                                .to_string()
                        })
                        .collect();
                    if contents.iter().any(|d| constants::SEASON_RE.is_match(d)) {
                        cnt += 1;
                    }
                }
            }
            println!(
                "{}{}{}",
                "You have ".italic(),
                cnt.to_string().bold(),
                " television series in your Plex Media Server.".italic()
            );
        }
        MediaType::Unknown | MediaType::Impossible => {
            panic!("Unhandled media type");
        }
    }
}

fn count_media_files(dir: &Path) -> usize {
    // TODO: use walkdir or something (see jakewilliami/lsext)
    fn recurse_files_count_if_media(dir: &Path, cnt: &mut isize) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    recurse_files_count_if_media(&path, cnt)?;
                } else {
                    let ext = path::get_extension_from_filename(&path);
                    if ext.is_some() && constants::MEDIA_TYPES.contains(&ext.unwrap()) {
                        *cnt += 1;
                    }
                }
            }
        }
        Ok(())
    }
    let mut cnt = 0;
    let _ = recurse_files_count_if_media(dir, &mut cnt);
    cnt.try_into().unwrap()
}
