# eBPF utils

### Generate a  probe :

```shell
./generate "__x64_sys_tgkill" --kprobe --args "sys_tgid:%d" "sys_pid:%d" "sys_signal:%d" -o /tmp/tgkill.c --prober-name "prober-tgkill.yaml"
```    

```
generate kprobe probe : ["__x64_sys_tgkill"]
probers : Prober {
     probe_type: "kprobe",
     probe_path: "/tmp/tgkill.c",
     probe_init: "do_probing",
     arguments: [
         "sys_tgid:%d",
         "sys_pid:%d",
         "sys_signal:%d",
     ],
     map_object: "ebpf_map_events",
     map_object_type: "hashmap",
     probe_hook: "__x64_sys_tgkill",
     }
```

### Monitor with prober :

```shell
$ ./monitor --probes prober-tgkill.yaml
```

```
loading BPF program /tmp/tgkill.c into BPF VM...
monitor > all
source : /tmp/tgkill.c___x64_sys_tgkill
{
    "sys_tgid": "11435",
    "sys_pid": "11437",
    "sys_signal": "23",
    "comm": "test",
    "pid": "11443",
}
{
    "comm": "test",
    "sys_signal": "2",
    "sys_pid": "11435",
    "pid": "11435",
    "sys_tgid": "11435",
}
```

### Monitor with multiple probers :

#### generate a second probe

```shell
$ ./target/release/generate "__x64_sys_openat" --kprobe --args "sys_f:%d" "sys_path:%s@user" -o /tmp/openat.c --prober-name "prober-openat.yaml"
```

```
generate kprobe probe : ["__x64_sys_openat"]
probers : Prober {
    probe_type: "kprobe",
    probe_path: "/tmp/openat.c",
    probe_init: "do_probing",
    arguments: [
        "sys_f:%d",
        "sys_path:%s",
    ],
    map_object: "ebpf_map_events",
    map_object_type: "hashmap",
    probe_hook: "__x64_sys_openat",
}
```

```shell
$ ./monitor --probes prober-tgkill.yaml prober-openat.yaml
```

```
monitor > all
source : /tmp/tgkill.c___x64_sys_tgkill
{
    "sys_tgid": "11727",
    "comm": "test",
    "pid": "11728",
    "sys_pid": "11727",
    "sys_signal": "23",
}
{
    "comm": "test",
    "sys_signal": "2",
    "sys_tgid": "11727",
    "pid": "11727",
    "sys_pid": "11727",
}
source : /tmp/openat.c___x64_sys_openat
{
    "sys_path": "/proc/meminfo",
    "sys_f": "-100",
    "pid": "1569",
    "comm": "MemoryPoller",
}

```
