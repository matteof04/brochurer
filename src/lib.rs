/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
//! # Brochurer
//!
//! Calcultor for the page numbers to use for brochure printing.
//!
use std::fmt::{Display, Formatter};

pub mod utils;

/// Represent a printable one side sheet, containing the number of the two page that will be printed
#[derive(Clone, Copy, Debug)]
pub struct Page(u32, u32);

impl Page {
    /// Create a new sheet with the given pages number
    pub fn new(first: u32, second: u32) -> Page {
        assert_ne!(first, 0, "Start can't be 0!");
        Page(first, second)
    }
    /// Given the start and stop range, return the corresponding front `Page`
    pub fn front_page(start: u32, stop: u32) -> Page {
        Page::new(stop, start)
    }
    /// Given the start and stop range, return the corresponding back `Page`
    pub fn back_page(start: u32, stop: u32) -> Page {
        assert!(stop > 3, "Stop must be greater than 3!");
        Page::new(start + 1, stop - 1)
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{},{}", self.0, self.1)?;
        Ok(())
    }
}

/// Represent a printable two side sheet, containing the two `Page` that will be printed
#[derive(Clone, Copy, Debug)]
pub struct DuplexPage(Page, Page);

impl DuplexPage {
    /// Create a new `DuplexPage` with the given `Page`s
    pub fn new(first: Page, second: Page) -> DuplexPage {
        DuplexPage(first, second)
    }
    /// Given the start and stop range, return the corresponding `DuplexPage`
    pub fn from_sides(start: u32, stop: u32) -> DuplexPage {
        DuplexPage::new(Page::new(stop, start), Page::new(start + 1, stop - 1))
    }
}

impl Display for DuplexPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{},{}", self.0, self.1)?;
        Ok(())
    }
}
