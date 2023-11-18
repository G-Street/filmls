use lazy_static::lazy_static;
use regex::Regex;

// Define media source types

// Source: https://blog.filestack.com/thoughts-and-knowledge/complete-list-audio-video-file-formats/
pub const MEDIA_TYPES: [&str; 18] = [
    "mkv", "avi", "mp4", "webm", "mpg", "mp2", "mpeg", "mpe", "mpv", "ogg", "m4p", "m4v", "wmv",
    "mov", "qt", "flv", "swf", "avdchd",
];

// Source: https://github.com/seanap/Plex-Audiobook-Guide/blob/master/Scripts/BookCopy.sh
const _AUDIOBOOK_TYPES: [&str; 20] = [
    "m4b", "mp3", "mp4", "m4a", "ogg", "pdf", "epub", "azw", "azw3", "azw4", "doc", "docx", "m4v",
    "djvu", "opf", "odt", "pdx", "wav", "mobi", "xls",
];

pub const SUBTITLE_TYPES: [&str; 5] = ["srt", "smi", "ssa", "ass", "vtt"];

lazy_static! {
    pub static ref FILM_RE: Regex = Regex::new(r"^(?P<fname>.+)\s+\((?P<fyear>\d{4})\)$").unwrap();
    pub static ref SEASON_RE: Regex = Regex::new(r"^Season\s(?P<snum>\d{2,})(\s\-\s(?P<sname>.+))?$").unwrap();
    // pub static ref EP_RE: Regex = Regex::new(r"^(.*)\s\-\sS(\d+)E(\d+)(?:\s\-\s)(?:.*)\.(.*)$").unwrap();  // THIS WAS BUGGED - DOES NOT WORK!
    pub static ref EP_RE: Regex = Regex::new(r"^(?P<sname>.+)\s\-\sS(?P<snum>\d+)E(?P<epnum>\d{2,})(\s-\s)?(?P<epname>.+)?\.(?P<ext>\w+)$").unwrap();
    static ref SUB_EXT_RE: Regex = Regex::new(&SUBTITLE_TYPES.join("|")).unwrap();
    static ref SUB_RE_STR: String = format!(r"^(?P<fname>.+)\.(?P<locale>(\w{{2}}(\-\w{{2}})?)|\w{{3}})\.({})$", SUB_EXT_RE.to_string());
    pub static ref SUB_RE: Regex = Regex::new(&SUB_RE_STR).unwrap();
}
