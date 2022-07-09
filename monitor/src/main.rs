mod loader;
mod monitor;
mod probers;
mod session;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("ebpf-monitor")
        .about("monitoring ebpf probe generated with ebpf-util")
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
        .get_matches();

    let probes: Vec<probers::Prober> = match matches.values_of("probes") {
        Some(p) => {
            let mut probe = vec![];
            for p1 in p {
                let file = match std::fs::File::open(p1) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return;
                    }
                };

                let info: probers::Prober = match serde_yaml::from_reader(file) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return;
                    }
                };

                probe.push(info);
            }
            probe
        }
        None => vec![],
    };

    let binary: Option<String> = match matches.value_of("binary") {
        Some(b) => Some(b.to_string()),
        None => None,
    };

    let loader = loader::new(probes);
    loader::load(loader, binary);
    return;
}
