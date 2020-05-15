use std::io::prelude::*;
use std::{
    error::Error,
    ffi::OsString,
    fs::{self, File},
    path::{Path, PathBuf},
};

use crate::types::{self, Position, WorldMap};

mod csv_mapper;
mod mem_mapper;

pub use csv_mapper::CsvMapper;
pub use mem_mapper::MemMapper;

/// `WorldMap` loading interface
pub trait Mapper {
    /// Returns a `Result` that may have a WorldMap instance loaded from...somewhere.
    fn load_map(&self) -> Result<WorldMap, Box<dyn Error>>;
}

/// Get list of files in directory with .csv extension
fn get_csvs_from_dir(dir_path: &str) -> Vec<PathBuf> {
    list_dir_with_ext("csv")(dir_path)
}

/// Returns a function that takes a directory and lists files with given extension
fn list_dir_with_ext<'a>(extension: &'a str) -> Box<dyn Fn(&str) -> Vec<PathBuf> + 'a> {
    Box::new(move |dir_path| {
        fs::read_dir(dir_path)
            .unwrap_or_else(|e| panic!("Unable to read dir {}: {}", dir_path, e))
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap().path())
            .filter(|file_path| file_path.extension() == Some(&OsString::from(extension)))
            .collect()
    })
}

/// Load contents of a file into a String. **`panic!`s on errors.**
fn load_to_string(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    // open the path in read only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldnt open {}: {}", display, why),
        Ok(file) => file,
    };

    // read the contents into a string
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldnt open {}: {}", display, why);
    }

    s
}

/// parse string slices to u32, `panic!` on failure
fn parse_u32(s: &str) -> u32 {
    s.parse::<u32>()
        .unwrap_or_else(|_| panic!("Unable to parse {} to u32", s))
}

/// parse csv (as a `&str`) into a 2d Vec of `u32` values
fn csv_into_vec(s: &str) -> Vec<Vec<u32>> {
    s.split('\n')
        .filter(|ln| !ln.is_empty())
        .map(|str| {
            str.split(',')
                .map(|str| str.trim())
                .map(parse_u32)
                .collect()
        })
        .collect()
}

/// parse 2d Vec of `u32` values into a Vec of `Position`s
fn grid_to_position_vec(grid: Vec<Vec<u32>>) -> Vec<Position> {
    const BG_CELL: u32 = 1;
    let mut result = Vec::new();

    for (r, list) in (0..).zip(grid) {
        for (c, val) in (0..).zip(list) {
            if val == BG_CELL {
                result.push((r, c));
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_u32() {
        let expected: u32 = 123;
        let actual = parse_u32("123");
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn panics_on_parse_failure() {
        parse_u32("foo");
    }

    #[test]
    fn csv_to_vec() {
        let expected = vec![[1, 1], [0, 0]];
        let actual = csv_into_vec(
            "1,1
            0,0",
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn loads_to_string() {
        let expected = "1,1
0,0
";
        let actual = load_to_string("./fixture/map_00.csv");
        assert_eq!(actual, expected);
    }

    #[test]
    fn file_to_vec() {
        let expected = vec![[1, 1], [0, 0]];
        let s = load_to_string("./fixture/map_00.csv");
        let actual = csv_into_vec(&s);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_positions() {
        let expected = vec![(0, 0), (1, 1)];
        let data: Vec<Vec<u32>> = vec![vec![1, 0], vec![0, 1]];
        let actual = grid_to_position_vec(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn list_csv() {
        let expected = vec![PathBuf::from("./fixture/map_00.csv")];
        let actual = get_csvs_from_dir("./fixture");
        assert_eq!(actual, expected);
    }

    #[test]
    fn list_csv_curried() {
        let expected = vec![PathBuf::from("./fixture/map_00.csv")];
        let list_csvs = list_dir_with_ext("csv");
        let actual = list_csvs("./fixture");
        assert_eq!(actual, expected);
    }
}
