
use std::path::Path;

use clap::clap_app;
use ions::ion_to_json;

fn main() {

    let opt = Options::parse();

    match (opt.input.len(), opt.is_concat, opt.verbose, opt.recursive) {
        (0, _, _, _) => unreachable!(),
        (1, false, v, false) => {
            let s = opt.input.first().unwrap();
            if v {
                println!("transpiling {} to {}", s, opt.out);
            }
            let p = std::fs::read_to_string(s).unwrap();
            let p = ion_to_json(&p).unwrap();
            std::fs::write(opt.out, p).unwrap();
        },
        (1, true, _v, true) => {
            let p = all_files(opt.input.get(0).unwrap());
            let p = ion_to_json(&p).unwrap();
            std::fs::write(opt.out, &p).unwrap();
            println!("{}", p);
        },
        (_, false, v, false) => opt.input.iter().for_each(|s| {
            if v {
                println!("transpiling {} to {}.json", s, s);
            }
            let p = std::fs::read_to_string(s).unwrap();
            let p = ion_to_json(&p).unwrap();
            std::fs::write(format!("{}.json", s), p).unwrap();
        }),
        (_, true, _v, false) => {
            let mut buf = "".to_string();
            opt.input.iter().for_each(|s| {
                let p = std::fs::read_to_string(s).unwrap();
                buf.push_str(&p);
            });
            let p = ion_to_json(&buf).unwrap();
            std::fs::write(opt.out, p).unwrap();
        },
        (_, _, _v, true) => opt.input.iter().for_each(|s| {
            recurse_dir(s, &opt.out).unwrap();
        }),
    }

}


fn all_files<T: AsRef<Path>>(input: T) -> String {
    let mut buf = "".to_string();
    let d = std::fs::read_dir(&input).unwrap();
    d.into_iter().for_each(|s| {
        if let Ok(s) =  s {
            if let Ok(q) = s.file_type() {
                if q.is_dir() {
                    all_files(s.path());
                }
            }
            let name = s.file_name();
            if name.to_str().unwrap().contains(".ion") {
                let o = std::fs::read_to_string(&s.path()).unwrap();
                buf.push_str(&o);
            }
        }
    });
    buf
}

fn recurse_dir<T: AsRef<Path> + std::fmt::Display>(inp: T, out: T) -> Result<(), anyhow::Error> {
    let d = std::fs::read_dir(&inp).unwrap();
    d.into_iter().for_each(|s| {
        if let Ok(s) =  s {
            if let Ok(s) = s.file_type() {
                if s.is_dir() {

                }
            }
            let name = s.file_name();
            if name.to_str().unwrap().contains(".ion") {
                let o = std::fs::read_to_string(&s.path()).unwrap();
                let mut o = ions::ion_to_json(&o).unwrap();
                if o == "}" {
                    o = "".to_string();
                }
                std::fs::write(&format!("{}/{}.json", out, name.to_str().unwrap()), o).unwrap();
            }
        }
    });
    Ok(())
}

struct Options {
    input: Vec<String>,
    out: String,

    recursive: bool,
    is_concat: bool,
    verbose: bool,
}

impl Options {
    fn parse() -> Self {
        let matches = clap_app!(myapp =>
            (version: "1.0")
            (author: "Jose Salazar <znx3p0@gmail.com>")
            (about: "Transpiles ion files to json")
            (@arg RECURSIVE: -r --recursive "Sets recursive mode")
            (@arg IS_CONCAT: -c --concatenate "Sets concatenate mode")
            (@arg INPUT: ... * +required "Sets the input file to use")
            (@arg OUTPUT: -o --output +takes_value "Sets recursive mode")
            (@arg VERBOSE: -v --verbose "Verbose mode")
        ).get_matches();
        
        let input = if let Some(i) = matches.values_of("INPUT") {
            i.map(|s| s.to_string()).collect::<Vec<_>>() 
        } else {
            unreachable!()
        };

        let out = if let Some(i) = matches.value_of("OUTPUT") {
            i.to_string()
        } else {
            "".to_string()
        };

        let recursive = if matches.is_present("RECURSIVE") {
            true
        } else {
            false
        };

        let verbose = if matches.is_present("VERBOSE") {
            true
        } else {
            false
        };

        let is_concat = if matches.is_present("IS_CONCAT") {
            true
        } else {
            false
        };

        Self {
            input,
            out,
            recursive,
            is_concat,
            verbose,
        }
    }
}


