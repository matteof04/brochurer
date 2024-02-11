/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 * 
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    ///Calculate page numbers
    Calc(CalcArgs),
    ///Build autocomplete scripts for all the shells supported and save them into the complete folder
    BuildComplete,
}
#[derive(Args)]
pub(crate) struct CalcArgs {
    ///Page count
    pub(crate) page_count: u32,
    ///Chunk number
    #[clap(default_value_t = 1)]
    pub(crate) chunk: u32,
    ///Page numbering offset
    #[clap(default_value_t = 0)]
    pub(crate) offset: u32,
}

pub(crate) mod utils {
    use std::{
        fs::{create_dir, File},
        io::Write,
        path::Path,
    };

    use clap::{CommandFactory, ValueEnum};
    use clap_complete::Shell;

    use crate::cli;

    pub fn build_complete_file() {
        const BIN_NAME: &str = env!("CARGO_BIN_NAME");
        let base_dir = Path::new("complete");
        if !base_dir.exists() || !base_dir.is_dir() {
            create_dir(base_dir).unwrap_or_else(|_| panic!("Can't create the complete directory!"));
        }
        for shell in Shell::value_variants() {
            let file_name = format!(
                "{}/{BIN_NAME}.{shell}",
                base_dir.file_name().unwrap().to_str().unwrap()
            );
            let file_path = Path::new(&file_name);
            let mut file = File::create(file_path)
                .unwrap_or_else(|_| panic!("Can't create the complete file {file_name}!"));
            clap_complete::generate(
                shell.to_owned(),
                &mut cli::Cli::command(),
                BIN_NAME,
                &mut file,
            );
            println!("Generated complete file of {BIN_NAME} for {shell}");
        }
        let load_script = include_str!("../load_script_template");
        let load_script = load_script.replace(
            "COMPLETE_DIR",
            base_dir.file_name().unwrap().to_str().unwrap(),
        );
        let load_script = load_script.replace("BIN_NAME", BIN_NAME);
        let mut file = File::create("load").expect("Can't create the load file!");
        file.write_all(load_script.as_bytes())
            .expect("Can't write the load script!");
        println!("Generated load script");
    }
}
