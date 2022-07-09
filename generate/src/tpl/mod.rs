pub const MATCH_PROBE_NAME : &str = "/PROBE_NAME/";
pub const MATCH_PROBE_CALL : &str = "/PROBE_CALL/";
pub const MATCH_ASKED_VARIABLES : &str = "/ASKED_VARIABLES/";
pub const MATCH_USER_ARGUMENTS : &str = "/USER_ARGUMENT/";
pub const MATCH_NAME_MAP : &str = "/NAME_MAP/";

pub fn get_template() -> Vec<String> {
    let mut template = vec![];
    template.push("#include <linux/ptrace.h>".to_string());
    template.push("#include <linux/sched.h>".to_string());
    template.push("struct /PROBE_NAME/_data_t { ".to_string());
    template.push("u32 tgid;".to_string());
    template.push("u32 pid;".to_string());
    template.push("u32 uid;".to_string());
    template.push("char comm[50];".to_string());
    template.push("/ASKED_VARIABLES/".to_string());
    template.push("};".to_string());

    template.push("BPF_HASH(/NAME_MAP/_events, struct /PROBE_NAME/_data_t);".to_string());
    template.push("int /PROBE_CALL/(struct pt_regs *ctx) {".to_string());
    template.push("u64 __pid_tgid = bpf_get_current_pid_tgid();".to_string());
    template.push("u32 __tgid = __pid_tgid >> 32;".to_string());
    template.push("u32 __pid = __pid_tgid;".to_string()); // implicit cast to u32 for bottom half
    template.push("u32 __uid = bpf_get_current_uid_gid();".to_string());

    template.push("struct /PROBE_NAME/_data_t __data = {0};".to_string());
    template.push("__data.tgid = __tgid;".to_string());
    template.push("__data.pid = __pid;".to_string());
    template.push("__data.uid = __uid;".to_string());
    template.push("u64 key = 0;".to_string());
    template.push("bpf_get_current_comm(&__data.comm, sizeof(__data.comm));".to_string());
    template.push("/USER_ARGUMENT/".to_string());

    template.push("/NAME_MAP/_events.lookup_or_init(&__data, &key);".to_string());
    template.push("return 0;".to_string());
    template.push("}".to_string());
    template
}
