use clap::{Arg, Command};

fn main() {
    let matches = Command::new("ebpf-utils")
        .about("generate a eBPF probe or monitor them")
        .author("graniet")
        .subcommand_required(true)
        .subcommand(
            Command::new("generate")
                .about("generate a new (u/k)(ret)probe")
                .arg(
                    Arg::new("kprobe")
                        .required_unless_present("uprobe")
                        .takes_value(false)
                        .long("kprobe"),
                )
                .arg(
                    Arg::new("uprobe")
                        .required_unless_present("kprobe")
                        .takes_value(false)
                        .long("uprobe"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .required(true)
                        .long("output")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("args")
                        .long("args")
                        .short('a')
                        .required(false)
                        .multiple_values(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("prober")
                        .long("prober-name")
                        .help("custom name for generated prober template")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::new("probes").required(true).multiple_values(true)),
        )
        .subcommand(
            Command::new("monitor")
                .about("inject and monitor probe event")
                .about("graniet")
                .arg(
                    Arg::new("probes")
                        .short('p')
                        .long("probes")
                        .help("loading probes into kernel BPF VM")
                        .required(true)
                        .multiple_values(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("binary")
                        .long("binary")
                        .help("binary for userspace probe")
                        .takes_value(true),
                )
        )
        .get_matches();

    let action = matches.subcommand_name().unwrap();
    let subcommand = matches.subcommand_matches(action.clone()).unwrap();
    match action {
        "monitor" => monitor::process(subcommand),
        _ => generate::process(subcommand)
    }
}
