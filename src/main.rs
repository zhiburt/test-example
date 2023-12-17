use std::{collections::HashMap, path::PathBuf};

use log::info;

mod cli;
mod format;
mod run;

fn main() {
    env_logger::init();

    let args = cli::parse();

    info!("Parsing files: {:?}", args.file);
    info!("With a given libraries: {:?}", args.include);

    let mut examples = HashMap::new();

    for file in args.file {
        let path = PathBuf::from(&file);
        let mut file_examples =
            format::parse(&path).unwrap_or_else(|_| panic!("can't open a file [{:?}]", file));

        if !args.include.is_empty() {
            for example in &mut file_examples {
                example.include.extend(args.include.clone());
            }
        }

        for example in file_examples {
            match examples.contains_key(&file) {
                false => {
                    examples.insert(file.clone(), vec![example]);
                }
                true => {
                    let list = examples.get_mut(&file).unwrap();
                    list.push(example);
                }
            }
        }
    }

    for (file, examples) in examples {
        for (i, mut example) in examples.into_iter().enumerate() {
            // todo: see run::run (use fork when run)
            // require some IPC to pass the output?
            example.target = crate::format::Target::Build;

            let result = run::run(example).unwrap();
            let output = String::from_utf8(result.build).unwrap();
            println!("{:?} {} {:?}", file, i, output);
        }
    }
}
