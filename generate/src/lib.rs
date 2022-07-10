mod probe;
mod tpl;

use clap;

pub fn process(matches: &clap::ArgMatches) {

    let output = matches.value_of("output").unwrap();
    let probes : Vec<String> = match matches.values_of("probes") {
        Some(p) => {
            let mut v = vec![];
            for probe in p {
                v.push(probe.to_string());
            }
            v
        },
        None => vec![],
    };
    let mut arguments = vec![];

    if matches.is_present("args") {
        arguments = match matches.values_of("args") {
            Some(args) => {
                let mut lists = vec![];
                for arg in args {
                    lists.push(arg.to_string());
                }
                lists
            },
            None => vec![]
        }
    }


    let mut mode = "uprobe";

    let prober_name = matches.value_of("prober").unwrap();

    if matches.is_present("kprobe") {
        mode = "kprobe";
    }

    probe::new(mode, probes, output.to_string(), arguments, prober_name.to_string());
}
