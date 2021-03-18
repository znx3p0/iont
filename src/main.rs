
use std::env::args;

fn main() {

    let args = args().collect::<Vec<_>>();
    if args.len() == 1 {
        usage();
        std::process::exit(1);
    }

    let mut args = args.into_iter();
    args.next();
    let args = parse_args(args.into_iter());
    
    run(args)
}

fn parse_args(mut a: std::vec::IntoIter<String>) -> Options {

    // let fi = a.next();
    // let se = a.next();
    // let th = a.next();
    // let fo = a.next();

    // let p = match (fi, se, th, fo) {
    //     (Some(format!("-d")), Some(input), Some("-o"), Some(out)) => Options {
    //         input,
    //         out,
    //     },
    //     (Some(input), Some("-o".to_string()), Some(out), None) => Options {
    //         input,
    //         out,
    //     },
    //     (Some(input), Some(out), None, None) => Options {
    //         input,
    //         out,
    //     },
    //     (None, None, None, None) => {
    //         usage();
    //         std::process::exit(1);
    //     }
    // };

    Options {
        input: a.next().unwrap(),
        out: a.next().unwrap()
    }

}

fn run(args: Options) {
    println!("{}", args.input);

    if std::fs::read_dir(&args.input).is_ok() {
        // input is dir
        if std::fs::read_dir(&args.out).is_ok() {
            // output is dir
            let d = std::fs::read_dir(&args.input).unwrap();
            d.into_iter().for_each(|s| {
                if let Ok(s) =  s {
                    let name = s.file_name();
                    if name.to_str().unwrap().contains(".ion") {
                        let o = std::fs::read_to_string(&s.path()).unwrap();
                        let o = ions::ion_to_json(&o).unwrap();
                        std::fs::write(&format!("{}/{}.json", args.out, name.to_str().unwrap()), o).unwrap();
                        println!("Successfully transpiled ion to json")
                    }
                }
            });

        } else {
            // output is file

            let mut buf = "".to_string();
            let d = std::fs::read_dir(&args.input).unwrap();
            d.into_iter().for_each(|s| {
                if let Ok(s) =  s {
                    let name = s.file_name();
                    if name.to_str().unwrap().contains(".ion") {
                        let o = std::fs::read_to_string(&s.path()).unwrap();
                        buf.push_str(&format!("{}\n", o));

                    }
                }
            });
            
            let o = ions::ion_to_json(&buf).unwrap();
            std::fs::write(args.out, o).unwrap();
        }
        

    } else {
        // input is file

        let o = std::fs::read_to_string(args.input).unwrap();
        let o = ions::ion_to_json(&o).unwrap();
        std::fs::write(args.out, o).unwrap();
        println!("Successfully transpiled ion to json")
    }
}

fn usage() {
    println!(r#"
iont | ion transpiler

usage: 
    iont <file> <out|optional>
        flags:
        -d <dir>
        -o <out>
"#)
}

struct Options {
    input: String,
    out: String,
}


