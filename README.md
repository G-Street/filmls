<h1 align="center">filmls</h1>

## Description

A command line interface for listing films in order of date, and other utilities for interacting with/viewing our NAS media at a glance.

This was ported originally from a [Bash script](https://github.com/G-Street/media-scripts/blob/master/bash/countMedia) into Rust, and has since migrated from its [original repo](https://github.com/jakewilliami/scripts/tree/master/rust/filmls/).

## Quick Start

```shell
$ ./compile.sh
$ ./filmls -h
```

## Usage

```shell
$ filmls -h
A command line interface for listing films in order of date

Usage: filmls [OPTIONS] [DIR]

Arguments:
  [DIR]  Takes an input directory.  Omitting this parameter, the programme will attempt to find the media directory

Options:
  -f, --films                Look in the film directory.  You can use this flag with -c
  -s, --series               Look in the series directory.  You can use this flag with -c
  -c, --count                Count the number of films or series in a directory.  Choose -f or -s for the programme to find the directory for you, otherwise specify a directory
  -t, --titles               Check if series have titles for each episode
  -S, --consecitive-seasons  Check if series have consecutive seasons
  -e, --complete-episodes    Check if series have all episodes in each season
  -h, --help                 Print help information
  -V, --version              Print version information

$ filmls  # With no arguments, will do its original purpose: list media (sorted by date)
Orders to Kill (1958)
Young Frankenstein (1974)
# ...

$ filmls --count --films
You have 62 films in your Plex Media Server.

$ filmls --count --series
You have 35 television series in your Plex Media Server.
```
