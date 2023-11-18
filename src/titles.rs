use std::collections::HashMap;
use std::fs;
use std::path::Path;

use colored::*;

use super::constants;
use super::dir::MediaType;

pub fn check_series_titles(dirname: &Path) {
    let mut series_dir = dirname.to_path_buf();
    series_dir.push(MediaType::Series.as_str());
    // Construct a hashmap for storing results
    let mut missing_ep_names_map = HashMap::<String, Vec<isize>>::new();
    // Get series available
    let series: Vec<_> = fs::read_dir(&series_dir)
        .unwrap_or_else(|_| panic!("Cannot read directory: {:?}", series_dir))
        .map(|e| e.expect("Cannot retreive file information").path())
        .collect();
    // Search through series
    for path in series {
        let series_name_outer = &path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_dir() {
            missing_ep_names_map.insert(series_name_outer.to_string(), vec![]);
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
            // Search through series' seasons
            for season_dir in contents.iter().filter(|d| constants::SEASON_RE.is_match(d)) {
                let mut season_dir_path = path.clone();
                season_dir_path.push(season_dir);
                let season_content: Vec<_> = fs::read_dir(&season_dir_path)
                    .expect("Cannot read directory")
                    .map(|e| {
                        e.expect("Cannot retrieve file information")
                            .path()
                            .file_name()
                            .expect("Cannot get file name from file")
                            .to_str()
                            .unwrap()
                            .to_string()
                    })
                    .collect();
                // Search through episodes
                for ep in season_content {
                    if constants::EP_RE.is_match(&ep) {
                        let caps = constants::EP_RE.captures(&ep).unwrap();
                        // Check if episode has a name
                        if caps.name("epname").is_none() {
                            let series_name = caps.name("sname").unwrap().as_str().to_string();
                            let season_num = caps
                                .name("snum")
                                .unwrap()
                                .as_str()
                                .parse::<isize>()
                                .unwrap();
                            if let Some(v) = missing_ep_names_map.get_mut(&series_name) {
                                (*v).push(season_num);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    // Display results
    for (s, v) in missing_ep_names_map.iter() {
        if !v.is_empty() {
            println!("{}", &s.blue().bold())
        }
        let mut w = v.clone();
        w.sort();
        for si in w.iter() {
            println!("\t{}{}", "Season ".blue(), si.to_string().blue())
        }
    }
}
