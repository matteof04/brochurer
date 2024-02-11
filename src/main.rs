/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

use brochurer::utils::calc;
use clap::Parser;
use cli::utils::build_complete_file;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Calc(args) => {
            if args.page_count % 4 != 0 {
                eprintln!("Page count must be a multiple of 4!");
            }
            if args.chunk == 0 {
                eprintln!("Chunk can't be 0!");
            }
            if args.page_count % args.chunk != 0 {
                eprintln!("Page count must be a multiple of chunk!");
            }
            calc(args.page_count, args.chunk, args.offset);
        }
        cli::Commands::BuildComplete => build_complete_file(),
    }
}
