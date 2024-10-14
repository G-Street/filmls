use super::constants;
use super::dir::MediaType;
use super::path;
use colored::*;
use regex::Regex;
use std::path::Path;
use std::{fs, io, process};

pub fn list_erroneous_subtitles(dirname: &Path, media_type: &MediaType) {
    match media_type {
        MediaType::Film => {
            let mut films_dir = dirname.to_path_buf();
            films_dir.push(media_type.as_str());
            for sub in list_subtitles(&films_dir) {
                // TODO: warn if subtitle base name does not match film
                if !check_subtitle_format(&sub, &constants::SUB_RE) {
                    println!(
                        "{}{}{}",
                        "Subtitle file ".italic(),
                        sub.bold(),
                        " is incorrectly formatted".italic()
                    );
                }
            }
        }
        MediaType::Series => {
            let mut series_dir = dirname.to_path_buf();
            series_dir.push(media_type.as_str());

            // Exit gracefully if the series directory doesn't exist
            // Rather than panicking in the next step
            if !series_dir.exists() {
                eprintln!("No such file or directory: {:?}", series_dir);
                process::exit(1);
            }

            // Get individual series from series directory
            let series: Vec<_> = fs::read_dir(&series_dir)
                .unwrap_or_else(|_| panic!("Cannot read directory: {:?}", series_dir))
                .map(|e| e.expect("Cannot retreive file information").path())
                .collect();

            // Check subtitle format for each season for each series
            for path in series {
                if path.is_dir() {
                    for p in fs::read_dir(&path)
                        .expect("Cannot read directory")
                        .map(|e| e.unwrap())
                    {
                        let p = &p.path();
                        let p = &p.as_path();
                        let d = &p
                            .file_name()
                            .expect("Cannot get file name from file")
                            .to_str()
                            .unwrap();
                        if constants::SEASON_RE.is_match(d) {
                            for sub in list_subtitles(p) {
                                // TODO: warn if subtitle base name does not match film
                                if !check_subtitle_format(&sub, &constants::SUB_RE) {
                                    println!(
                                        "{}{}{}",
                                        "Subtitle file ".italic(),
                                        sub.bold(),
                                        " is incorrectly formatted".italic()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        MediaType::Unknown | MediaType::Impossible => {
            panic!("Unhandled media type");
        }
    }
}

fn list_subtitles(dir: &Path) -> Vec<String> {
    // First check that the path exists
    if !dir.exists() {
        eprintln!("No such file or directory: {:?}", dir);
        process::exit(1);
    }

    // TODO: use walkdir or something (see jakewilliami/lsext)
    fn recurse_files_count_if_media(dir: &Path, subs: &mut Vec<String>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    recurse_files_count_if_media(&path, subs)?;
                } else {
                    let ext = path::get_extension_from_filename(&path);
                    if ext.is_some() && constants::SUBTITLE_TYPES.contains(&ext.unwrap()) {
                        subs.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        }
        Ok(())
    }
    let mut subs = Vec::new();
    let _ = recurse_files_count_if_media(dir, &mut subs);
    subs
}

fn check_subtitle_format(sub: &str, pattern: &Regex) -> bool {
    pattern.is_match(sub)
}
