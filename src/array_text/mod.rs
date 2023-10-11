//! This module provides a parser to retrieve arrays of a uniform type, e.g. only f64 or only i32
//! seperated by a given seperator from a file.

use nalgebra::DMatrix;
use std::{io::BufRead, str::FromStr};

pub fn parse_text_into_matrix<I, T>(
    source: &mut I,
    separator: &str,
    ncols: usize,
    nrows: usize,
) -> DMatrix<T>
where
    I: BufRead + std::fmt::Debug,
    T: FromStr + std::fmt::Debug + nalgebra::Scalar,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut result: Vec<T> = Vec::new();
    let splitter = str::split_whitespace;
    if separator != " " {
        let splitter = str::split::<char>;
        unimplemented!("Splitting by another separator is not implemented, yet!");
    }
    let line_iter = source.lines().map(|l| l.unwrap());
    for line in line_iter {
        let sep_split = splitter(&line);
        for ele in sep_split {
            result.push(ele.parse::<T>().unwrap());
        }
    }
    DMatrix::from_vec(nrows, ncols, result)
}
