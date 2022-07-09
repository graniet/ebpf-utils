mod probe;
mod tpl;

use {
    clap::{
        Command,
        Arg,
    }
};

fn main() {
    let matches = Command::new("create")
        .about("generate eBPF (ret)probe C code for kernel/user land")
        .arg(
            Arg::new("kprobe")
                .required_unless_present("uprobe")
                .takes_value(false)
                .long("kprobe")
        )
        .arg(
            Arg::new("uprobe")
                .required_unless_present("kprobe")
                .takes_value(false)
                .long("uprobe")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .required(true)
                .long("output")
                .takes_value(true)
        )
        .arg(
            Arg::new("args")
                .long("args")
                .short('a')
                .required(false)
                .multiple_values(true)
                .takes_value(true)
        )
        .arg(
            Arg::new("prober")
                .long("prober-name")
                .help("custom name for generated prober template")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("probes")
                .required(true)
                .multiple_values(true)
        ).get_matches();


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
