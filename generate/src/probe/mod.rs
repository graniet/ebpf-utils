mod probers;

use {crate::tpl, std::io::prelude::*};

pub const FORMAT_INT: &str = "%d";
pub const FORMAT_STR: &str = "%s";
pub const READ_KERNEL: &str = "bpf_probe_read_kernel";
pub const READ_USER: &str = "bpf_probe_read_user";

// generate reading buffer for decimal argument (%d)
pub fn gen_integer_buffer(name: String, buffer_function: &str, key: usize) -> String {
    format!("__data.{} = (int)({{u64 _val; struct pt_regs *_ctx = (struct pt_regs *)PT_REGS_PARM1(ctx);  {}(&_val, sizeof(_val), &(PT_REGS_PARM{}(_ctx))); _val;}});", name, buffer_function, key)
}

// generate reading buffer for string argument (%s)
pub fn gen_string_buffer(name: String, buffer_function: &str, key: usize) -> String {
    format!("        if (({{ u64 _val; struct pt_regs *_ctx = (struct pt_regs *)PT_REGS_PARM1(ctx);  bpf_probe_read_kernel(&_val, sizeof(_val), &(PT_REGS_PARM{}(_ctx))); _val;}}) != 0) {{
                void *__tmp = (void *)({{u64 _val; struct pt_regs *_ctx = (struct pt_regs *)PT_REGS_PARM1(ctx);  bpf_probe_read_kernel(&_val, sizeof(_val), &(PT_REGS_PARM{}(_ctx))); _val;}});
                {}(&__data.{}, sizeof(__data.{}), __tmp);
        }}", key, key, buffer_function, name, name)
}

pub fn new(
    mode: &str,
    probes: Vec<String>,
    output: String,
    arguments: Vec<String>,
    prober_name: String,
) {
    for probe in probes.clone() {

        let probe_name = probe.clone();
        let mut probers = probers::Prober {
            probe_type: mode.to_lowercase().clone(),
            probe_path: output.clone(),
            probe_init: "do_probing".to_string(),
            arguments: vec![],
            map_object: "ebpf_map_events".to_string(),
            map_object_type: "hashmap".to_string(),
            probe_hook: probe.clone().to_string(),
        };

        let lines = tpl::get_template();
        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(output)
        {
            Ok(f) => f,
            Err(e) => {
                println!("ERR {}", e.to_string());
                return;
            }
        };

        let mut struct_variables = vec![];
        let mut argument_buffer = vec![];

        let valid_format = vec![FORMAT_INT, FORMAT_STR];

        for (mut key, argument) in arguments.clone().iter().enumerate() {
            let mut buffer_function = READ_KERNEL;
            key = key + 1;
            if !argument.contains(':') {
                println!(
                    "ERR invalid argument format, please use : arg1:({})@(user, kernel)",
                    valid_format.join(",")
                );
                return;
            }


            let info: Vec<&str> = argument.split(':').collect();
            let (name, mut format) = (info[0].to_lowercase(), info[1].to_lowercase());
            if format.contains("@") {
                let info = format.clone();
                let detail : Vec<&str> = info.split('@').collect();
                format = detail[0].to_lowercase();
                if detail[1].to_lowercase().contains("user") {
                    buffer_function = READ_USER;
                }
            }
            if !valid_format.contains(&format.as_str()) {
                println!(
                    "format unknown for argument {} please use : {}",
                    name,
                    valid_format.join(",")
                );
                return;
            }

            probers.arguments.push(format!("{}:{}", name, format));
            if format.eq(FORMAT_INT) {
                // generate variable u32 for decimal (%d) argument.
                struct_variables.push(format!("u32 {};", name));

                // generate a copy from buffer function
                let buffer_line = gen_integer_buffer(name, buffer_function, key);

                // push argument into buffer listing
                argument_buffer.push(buffer_line);
                continue;
            }

            // generate variable char for string (%s) argument.
            struct_variables.push(format!("char {}[80];", name));

            // generate a copy from buffer function
            let buffer_line = gen_string_buffer(name, buffer_function, key);

            // push argument into buffer listing
            argument_buffer.push(buffer_line);
            continue;
        }

        for mut line in lines {
            if line.to_uppercase().eq(tpl::MATCH_ASKED_VARIABLES) {
                for elem in &struct_variables {
                    let _ = writeln!(file, "{}", elem);
                }
                continue;
            }

            if line.to_uppercase().contains(tpl::MATCH_PROBE_NAME) {
                let final_name = probe_name.clone().replace(".", "_");
                line = line.replace(tpl::MATCH_PROBE_NAME, &final_name);
            }

            if line.to_uppercase().contains(tpl::MATCH_NAME_MAP) {
                line = line.replace(tpl::MATCH_NAME_MAP, "ebpf_map");
            }

            if line.to_uppercase().contains(tpl::MATCH_PROBE_CALL) {
                line = line.replace(tpl::MATCH_PROBE_CALL, "do_probing");
            }

            if line.to_uppercase().eq(tpl::MATCH_USER_ARGUMENTS) {
                for elem in &argument_buffer {
                    let _ = writeln!(file, "{}", elem);
                }
                continue;
            }

            let _ = writeln!(file, "{}", line);
        }
        println!("generate {} probe : {:?}", mode, probes);

        // export probers
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(prober_name)
            .unwrap();

        let content = serde_yaml::to_string(&probers).unwrap();
        let _ = write!(file, "{}", content);
        println!("probers : {:#?}", probers);

        return;
    }
}
