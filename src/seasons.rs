use std::collections::HashMap;
use std::fs;
use std::path::Path;

use colored::*;

use super::constants;
use super::dir::MediaType;

pub fn check_consecutive_seasons(dirname: &Path) {
    let mut series_dir = dirname.to_path_buf();
    series_dir.push(MediaType::Series.as_str());
    // Construct a hashmap for storing results
    let mut missing_seasons_map = HashMap::<String, Vec<isize>>::new();
    // Get series available
    let series: Vec<_> = fs::read_dir(&series_dir)
        .unwrap_or_else(|_| panic!("Cannot read directory: {:?}", series_dir))
        .map(|e| e.expect("Cannot retreive file information").path())
        .collect();
    // Search through series
    for path in series {
        let series_name_outer = &path.file_name().unwrap().to_str().unwrap().to_string();
        if path.is_dir() {
            missing_seasons_map.insert(series_name_outer.to_string(), vec![]);
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
            // Collect the season numbers within the series
            let season_numbers: Vec<isize> = contents
                .iter()
                .filter_map(|d| {
                    if constants::SEASON_RE.is_match(d) {
                        let caps = constants::SEASON_RE.captures(d).unwrap();
                        caps.name("snum")
                            .map(|season_num| season_num.as_str().parse::<isize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            // Check if these are consecutive
            let max_se_num = season_numbers.iter().max().unwrap_or_else(|| {
                panic!(
                    "Cannot determine maximum season number from seasons {:?}",
                    contents
                )
            });
            for i in 1..=*max_se_num {
                if !season_numbers.contains(&i) {
                    if let Some(v) = missing_seasons_map.get_mut(series_name_outer) {
                        (*v).push(i);
                    }
                }
            }
        }
    }
    // Display results
    for (s, v) in missing_seasons_map.iter() {
        if !v.is_empty() {
            println!("{}", &s.blue().bold())
        }
        let mut w = v.clone();
        w.sort();
        for si in w.iter() {
            println!("\t{}{}", "Missing Season ".blue(), si.to_string().blue())
        }
    }
}
