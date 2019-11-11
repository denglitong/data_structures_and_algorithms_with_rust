use std::path::Path;
use std::{fs, io};

/// Rust has no typical null pointer/reference
/// Instead, Rust works with ```Option``` and ```Result``` types
/// There is no exception system either, only in rare cases when immediate
/// termination is required does the Rust provide a macro for panicking ```panic!```.
/// Option<T> and Result<T> both being enumerations that have generic type parameters;
/// they can assume any type in their variants, and matching on their variants provides access to
/// their inner values and types to allow a branch of the code to be executed and handle the case accordingly.

fn find(needle: u16, haystack: Vec<u16>) -> Option<usize> {
    for (i, &item) in haystack.iter().enumerate() {
        if item == needle {
            return Some(i);
        }
    }
    None
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let path = Path::new(path);
    fs::read_to_string(&path)
}

#[cfg(test)]
mod tests {
    use crate::errors::{find, read_file};

    #[test]
    fn test1() {
        match find(2, vec![1, 3, 4, 5]) {
            Some(_) => println!("Found!"),
            None => println!("Not found :("),
        }

        if let Some(result) = find(2, vec![1, 3, 4, 5]) {
            println!("Found!");
        }

        match read_file("/tmp/not/a/file") {
            Ok(content) => println!("{}", content),
            Err(error) => println!("Oh no!"),
        }
    }
}
