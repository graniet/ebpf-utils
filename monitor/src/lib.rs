mod loader;
mod monitor;
mod probers;
mod session;

use clap;

pub fn process(matches: &clap::ArgMatches) {

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
