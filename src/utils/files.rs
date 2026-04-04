#![allow(dead_code)]
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn read_lines_iterable<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub trait Instantiable {
    fn new(values: Vec<&str>) -> Self;
}

pub fn read_lines_into_structs<T, F>(filename: &str, separator: Regex, mut retain_line: F) -> Vec<T>
where
    T: Instantiable,
    F: FnMut(String, i32) -> bool,
{
    let mut vec: Vec<T> = Vec::new();
    let mut i = 0;
    let lines_res = read_lines_iterable(filename);

    if let Ok(lines) = lines_res {
        for line in lines {
            if let Ok(l) = line {
                if retain_line(l.clone(), i) {
                    let values: Vec<&str> = separator.split(&l).collect();
                    let result = T::new(values);

                    vec.push(result);
                }

                i += 1;
            }
        }
    }

    return vec;
}

pub fn read_lines_into_lists_of_structs<T, F>(
    filename: &str,
    separator: Regex,
    list_separator: &str,
    retain_line: F,
) -> Vec<Vec<T>>
where
    T: Instantiable,
    F: Fn(String, i32) -> bool,
{
    let mut vec: Vec<Vec<T>> = Vec::new();
    let lines_res = read_lines_iterable(filename);
    let mut i = 0;
    let mut vec_pointer = 0;
    vec.push(Vec::new());

    if let Ok(lines) = lines_res {
        for line in lines {
            if let Ok(l) = line {
                if retain_line(l.clone(), i) {
                    if l == list_separator {
                        vec.push(Vec::new());
                        vec_pointer += 1;
                        continue;
                    }

                    let values: Vec<&str> =
                        separator.split(&l).filter(|&x| !x.is_empty()).collect();
                    let result = T::new(values);

                    vec[vec_pointer].push(result);
                }

                i += 1;
            }
        }
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() -> io::Result<()> {
        let mut lines = read_lines_iterable("test/test_input.txt")?;

        assert_eq!("forward 4", lines.next().unwrap().unwrap());
        assert_eq!("down 9", lines.next().unwrap().unwrap());
        assert_eq!("forward 6", lines.next().unwrap().unwrap());
        assert_eq!("down 5", lines.next().unwrap().unwrap());
        assert_eq!("up 2", lines.next().unwrap().unwrap());

        assert!(lines.next().is_none());

        Ok(())
    }

    #[test]
    fn test_read_lines_into_structs() -> io::Result<()> {
        #[derive(Debug, PartialEq)]
        struct Command {
            dir: String,
            value: i32,
        }

        impl Instantiable for Command {
            fn new(values: Vec<&str>) -> Self {
                Command {
                    dir: values[0].to_string(),
                    value: values[1].parse::<i32>().unwrap(),
                }
            }
        }

        let commands = read_lines_into_structs::<Command, _>(
            "test/test_input.txt",
            Regex::new(r"\s+").unwrap(),
            |_, _| true,
        );

        assert_eq!(
            commands[0],
            Command {
                dir: String::from("forward"),
                value: 4
            }
        );

        assert_eq!(
            commands[4],
            Command {
                dir: String::from("up"),
                value: 2
            }
        );

        Ok(())
    }

    #[test]
    fn test_read_lines_into_lists_of_structs() -> io::Result<()> {
        #[derive(Debug, PartialEq)]
        struct Pos {
            value: i32,
        }

        impl Instantiable for Pos {
            fn new(values: Vec<&str>) -> Self {
                Pos {
                    value: values[0].parse::<i32>().unwrap(),
                }
            }
        }

        let positions = read_lines_into_lists_of_structs::<Pos, _>(
            "test/test_input_2.txt",
            Regex::new(r"\s+").unwrap(),
            "",
            |_, i| i > 1,
        );

        assert_eq!(positions[0][0], Pos { value: 22 });

        assert_eq!(positions[2][2], Pos { value: 18 });

        Ok(())
    }
}
