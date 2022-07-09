use {
    serde::{Serialize, Deserialize}
};

// this structure contain declaration of eBPF probe
// work with eBPF-util generator
#[derive(Debug, Serialize, Deserialize)]
pub struct Prober {
    // what is a probe type ? (uprobe, kprobe, uretprobe, kretprobe)
    pub probe_type: String,
    // path for generated .c eBPF probe
    pub probe_path: String,
    // probe function init for hooker
    pub probe_init: String,
    // argument asked ?
    pub arguments: Vec<String>,
    // name of BPF map
    pub map_object: String,
    // BPF map type ? (event, HASH, ARRAY ... ?)
    pub map_object_type: String,
    // name of symbol target
    pub probe_hook: String,
}
