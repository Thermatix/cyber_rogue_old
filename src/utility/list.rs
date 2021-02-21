use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

pub fn from_file<P>(filename: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = r#try!(File::open(filename));
    Ok(io::BufReader::new(file).lines().map(|line| line.unwrap()).collect())
}
