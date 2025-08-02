use clap::{crate_authors, crate_name, crate_version, ArgAction, Parser, Subcommand};
use colored::*;
use std::{collections::HashMap, env, fs, path::PathBuf, process};

mod constants;
mod count;
mod dir;
mod episodes;
mod path;
mod seasons;
mod subtitles;
mod titles;

// TODO: clean up old code and make parts of this modular

// Define command line interface
#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
)]
/// A command line interface for listing films in order of date
struct Cli {
    /// Look in the film directory.  You can use this flag with -c
    #[arg(
        short = 'f',
        long = "films",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    films: Option<bool>,

    /// Look in the series directory.  You can use this flag with -c
    #[arg(
        short = 's',
        long = "series",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    series: Option<bool>,

    /// Count the number of films or series in a directory.  Choose -f or -s for the programme to find the directory for you, otherwise specify a directory
    #[arg(
        short = 'c',
        long = "count",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    count: Option<bool>,

    /// Check if series have titles for each episode
    #[arg(
        short = 't',
        long = "titles",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    titles: Option<bool>,

    /// Check if series have consecutive seasons
    #[arg(
        short = 'S',
        long = "consecitive-seasons",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    consecutive_seasons: Option<bool>,

    /// Check if series have all episodes in each season.  Use this flag with -f or -s
    #[arg(
        short = 'C',  // closed captions
        long = "subtitles",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    subtitles: Option<bool>,

    /// Check if film or series have correctly-formatted subtitles
    // TODO: why is this called complete episodes?
    #[arg(
        short = 'e',
        long = "complete-episodes",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    complete_episodes: Option<bool>,

    /// Takes an input directory.  Omitting this parameter, the programme will attempt to find the media directory
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
    )]
    dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Print the media directory the programme will use
    MediaDir,
}

// Main function

fn main() {
    let cli = Cli::parse();

    // If there is no directory provided, we will use either
    // a predefined media directory, or the current directory
    let dirname = if let Some(ref dirname) = cli.dir {
        dirname
    } else {
        &dir::get_media_dir()
    };

    match cli.command {
        Some(Command::MediaDir) => {
            println!("{}", dirname.display());
            process::exit(0);
        }
        None => {}
    }

    // Check that not both -s and -f are present
    if (cli.films, cli.series) == (Some(true), Some(true)) {
        eprintln!("[ERROR] Cannot infer media type when both -f and -s are given");
        process::exit(1);
    }

    // Construct media type from CLI args
    let mut media_type = None;
    if let Some(films) = cli.films {
        if films {
            media_type = Some(dir::MediaType::Film);
        }
    }
    if let Some(series) = cli.series {
        if series {
            media_type = Some(dir::MediaType::Series);
        }
    }
    if (cli.films, cli.series) == (Some(false), Some(false)) {
        media_type = Some(dir::MediaType::Impossible);
    }
    if media_type.is_none() && cli.films.is_none() && cli.series.is_none() {
        media_type = Some(dir::MediaType::Unknown);
    }
    let media_type = media_type.unwrap_or_else(|| {
        panic!(
            "Unhandled media type with (cli.films={:?}, cli.series={:?})",
            cli.films, cli.series
        )
    });

    // List films
    // If no arguments are passed, will list
    if std::env::args().len() <= 1 {
        // Get films directory if none provided
        let mut films_dir = dirname.clone();
        films_dir.push(dir::MediaType::Film.as_str());

        // We want to store films in a hashmap with <film name -> year> so that
        // we can sort it by year
        let mut film_map = HashMap::<String, isize>::new();
        // read the directory
        // Ensure it is sorted by title before we sort by date
        // We need to do this because the order in which `read_dir`
        // returns entries is not guaranteed.
        // If reproducible ordering is required the entries should be
        // explicitly sorted.
        let mut films: Vec<_> = fs::read_dir(&films_dir)
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

        films.sort();

        for film_name in films {
            // if there's a match, update the year of the film
            if constants::FILM_RE.is_match(&film_name) {
                let caps = constants::FILM_RE.captures(&film_name).unwrap();
                let film_year = caps
                    .name("fyear")
                    .unwrap()
                    .as_str()
                    .parse::<isize>()
                    .unwrap();
                film_map.insert(film_name, film_year);
            } else {
                eprintln!("Warning: film \"{}\" does not match regex", &film_name)
            }
        }

        // Convert film info to vector of tuples (which is inherently
        // ordered) and sort said vector by the year
        let mut film_vec: Vec<_> = film_map.iter().collect();
        film_vec.sort_by(|a, b| b.1.cmp(a.1));

        for f in film_vec.into_iter().rev() {
            println!("{}", f.0.blue().bold());
        }
    }

    // Count media
    if let Some(show_count) = cli.count {
        if show_count {
            count::show_count(dirname, &media_type);
        }
    }

    // Season utility functions
    // Check if season episodes have titles
    if let Some(check_titles) = cli.titles {
        if check_titles {
            titles::check_series_titles(dirname);
        }
    }

    // Alert on non-consecutive seasons
    if let Some(check_consecutive_seasons) = cli.consecutive_seasons {
        if check_consecutive_seasons {
            seasons::check_consecutive_seasons(dirname);
        }
    }

    // TODO: Check that no episodes are missing from any given season
    if let Some(check_complete_episodes) = cli.complete_episodes {
        if check_complete_episodes {
            episodes::check_complete_episodes(dirname);
        }
    }

    // Check subtitle format
    // https://github.com/G-Street/media-scripts/blob/4dfc232d/plex/format.md#subtitles
    if let Some(check_subtitles) = cli.subtitles {
        if check_subtitles {
            let films_only = (cli.films, cli.series) == (Some(true), Some(false));
            let series_only = (cli.films, cli.series) == (Some(false), Some(true));
            if !(films_only || series_only) {
                eprintln!("[ERROR] Must use -f or -s with -C");
                process::exit(1);
            }
            // TODO: print what the correct format should be?
            subtitles::list_erroneous_subtitles(dirname, &media_type)
        }
    }
}
