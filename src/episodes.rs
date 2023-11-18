use std::path::Path;

pub fn check_complete_episodes(_dirname: &Path) {
    todo!();
    /*
    let mut series_dir = dirname.to_path_buf();
    series_dir.push(SERIES_DIR_NAME);
    // Construct a hashmap for storing results
    let mut missing_eps_map = HashMap::<String, Vec<isize>>::new();
    // Get series available
    let series: Vec<_> = fs::read_dir(&series_dir)
        .expect(format!("Cannot read directory: {:?}", series_dir).as_str())
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
            for season_dir in contents.iter().filter(|d| season_re.is_match(d)) {
                let mut season_dir_path = path.clone();
                season_dir_path.push(&season_dir);
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
                    if ep_re.is_match(&ep) {
                        todo!("In this block, I need to get the episode capture group (3), and do a similar thing to that of the previous one, were we look for max value and find all missing values up to max.  Should print \"Series <series> has all consecutive episodes up to <max_ep>, if we don't find any missing.\"");
                        let caps = ep_re.captures(&ep).unwrap();
                        // Check if episode has fifth group (implying it must have a fourth pertaining to ep name)
                        if caps.get(5).is_none() {
                            let series_name = caps.get(1).unwrap().as_str().to_string();
                            let season_num =
                                caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
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
        } /*else {
              println!("{}", &s.green())
          }*/
        let mut w = v.clone();
        w.sort();
        for si in w.iter() {
            println!("\t{}{}", "Season ".blue(), si.to_string().blue())
        }
    }
    */
}
