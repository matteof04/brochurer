/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

//! Utility functions for the calculating and printing of page sets
use std::fmt::Display;

use crate::{DuplexPage, Page};

/// Utility to create a vector of `DuplexPage`s with the given page count and offset.
pub fn get_duplex_pages(count: u32, offset: u32) -> Vec<DuplexPage> {
    let mut pages = vec![];
    for i in (0..(count / 2)).step_by(2) {
        pages.push(DuplexPage::from_sides(i + 1 + offset, count - i + offset));
    }
    pages
}

/// Utility to split the vector of `DuplexPage` into a tuple with two vectors, one with the front and one with the back `Page`s.
pub fn get_front_back_pages(duplex: Vec<DuplexPage>) -> (Vec<Page>, Vec<Page>) {
    duplex.into_iter().map(|p| (p.0, p.1)).unzip()
}

/// Utility to get a string with all the element of a `Vec`. The element must implement `Display`.
/// If used with a vector of `Page` or `DuplexPage` it produces a comma separated page list, optimal as input for printing programs
pub fn get_string<T: Display>(data: Vec<T>) -> String {
    data.into_iter()
        .map(|d| format!("{d}"))
        .collect::<Vec<String>>()
        .join(",")
}

/// Utility to calculate and pretty print all the page sets, calculated with the given page count, offset and chunk count.
/// `Chunk` and `page_count` must be greater than zero. Also, `page_count` must be divisible by 4.
pub fn calc(page_count: u32, chunk: u32, offset: u32) {
    let chunk_size = page_count / chunk;
    for i in 0..chunk {
        let pages = get_duplex_pages(chunk_size + offset, chunk_size * i);
        if chunk != 1 {
            println!("Chunk: {}", i + 1);
        }
        print_pages(pages);
    }
}

/// Utility to pretty print a vector of `DuplexPage`s
pub fn print_pages(duplex_pages: Vec<DuplexPage>) {
    let pages = get_front_back_pages(duplex_pages.clone());
    let duplex_string = get_string(duplex_pages);
    let front_pages = pages.0;
    let back_pages = pages.1;
    let front_pages_string = get_string(front_pages);
    let back_pages_string = get_string(back_pages);
    println!("Duplex: {duplex_string}");
    println!("  Front: {front_pages_string}");
    println!("  Back: {back_pages_string}");
}
