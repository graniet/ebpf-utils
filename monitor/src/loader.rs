use crate::monitor::message::{TYPE_CONTEXT, TYPE_SESSION_ALL, TYPE_SESSION_REQ};
use crate::probers::Prober;
use crate::session::{self, SessionReq};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::mpsc;

pub const PROBE_USER_SPACE: &str = "uprobe";
pub const PROBE_KERNEL_SPACE: &str = "kprobe";

// Loader used for inject BPF probe into BPF kernel VM
pub struct Loader {
    // list of probers
    pub probers: Vec<Prober>,
}

// Context of probe event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadContext {
    // probe path
    pub probe: String,
    // probe main function
    pub hook: String,
    // returned data in bytes
    pub data: Vec<u8>,
    // argument asked
    pub args: Vec<String>,
}

// generate a loader with default state
pub fn new(probers: Vec<Prober>) -> Loader {
    Loader { probers }
}

pub fn load(loader: Loader, binary: Option<String>) {
    let (_session, _session_tx) = session::new();
    let (tx, rx) = mpsc::channel::<crate::monitor::message::Message>();

    println!("> monitor {} probes", loader.probers.len());
    for probe in loader.probers {
        println!(
            "> executing a prober : payload={}, type={}",
            probe.probe_path, probe.probe_type
        );

        let mut probe_file = match std::fs::File::open(probe.probe_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                println!("ERR {}", e.to_string());
                return;
            }
        };

        let mut content = String::new();
        if let Err(e) = probe_file.read_to_string(&mut content) {
            println!("ERR {}", e.to_string());
            return;
        };

        //let probe_text = include_str!("/home/graniet/projects/probe-guard/probes/example.c");
        let probe_text = content.as_str();
        let mut module = match bcc::BPF::new(probe_text) {
            Ok(m) => m,
            Err(e) => {
                println!("ERR {}", e.to_string());
                return;
            }
        };

        // clear screen after BPF program compilation
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("> loading BPF program {} into BPF VM...", probe.probe_path);
        match probe.probe_type.clone().as_str() {
            PROBE_USER_SPACE => {
                if binary.clone().is_none() {
                    println!("ERR please use --binary option for specified userspace task");
                    return;
                }
                bcc::Uprobe::new()
                    .handler(&probe.probe_init.clone().as_str())
                    .binary(binary.clone().unwrap())
                    .symbol(&probe.probe_hook.clone())
                    .attach(&mut module)
                    .unwrap();
            }
            PROBE_KERNEL_SPACE => {
                bcc::Kprobe::new()
                    .handler(&probe.probe_init.clone())
                    .function(&probe.probe_hook.clone())
                    .attach(&mut module)
                    .unwrap();
            }
            _ => {}
        }

        let current_tx = tx.clone();
        std::thread::spawn(move || {
            // loading BPF object
            let mut table = module.table(probe.map_object.clone().as_str()).unwrap();

            loop {
                // waiting 1000 milliseconds
                std::thread::sleep(std::time::Duration::from_millis(1000));

                // checking content of BPF object
                for e in &table {
                    // for each entry we load a context
                    let ctx = LoadContext {
                        probe: probe.probe_path.clone(),
                        hook: probe.probe_hook.clone(),
                        data: e.key.clone(),
                        args: probe.arguments.clone(),
                    };

                    // convert it to json
                    let json = match serde_json::to_value(&ctx) {
                        Ok(json) => json,
                        Err(e) => {
                            println!("ERR {}", e);
                            return;
                        }
                    };

                    // sending to monitor channel
                    _ = current_tx.send(crate::monitor::message::Message {
                        command_type: TYPE_CONTEXT,
                        command_content: json,
                    });

                    // deleting all data into table object (@todo usage of event map only)
                    _ = table.delete_all();
                }
            }
        });
    }

    // spawn monitor in thread
    std::thread::spawn(move || {
        crate::monitor::now(rx);
    });


    // creating a readline instance for interaction with human and probe commands
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("monitor > ");
        match readline {
            Ok(line) => {
                // parsing a line
                if !line.contains(":") {
                    if line.to_lowercase().eq("all") {
                        let _ = tx.send(crate::monitor::message::Message {
                            command_type: TYPE_SESSION_ALL,
                            command_content: serde_json::Value::Bool(true),
                        });
                    }
                    //println!("invalid command please use : hook:<hook> or comm:<comm> or pid:<dwefw> or probe:<probe.c>");
                    continue;
                }
                let command: Vec<&str> = line.split(":").collect();
                let mut content = SessionReq::default();
                content.value = command[1].to_string();
                let json = match serde_json::to_value(&content) {
                    Ok(json) => json,
                    Err(e)=> {
                        println!("ERR {}", e);
                        return;
                    }
                };
                let _ = tx.send(crate::monitor::message::Message {
                    command_type: TYPE_SESSION_REQ,
                    command_content: json,
                });
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
