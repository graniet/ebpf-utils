pub mod message;

use message::Message;
use crate::loader::LoadContext;
use crate::session::{SessionReq};
use serde_json::Value;
use std::collections::HashMap;
use std::ptr;
use std::sync::mpsc;


#[repr(C)]
#[derive(Debug)]
pub struct Event {
    pub tgid: i32,
    pub pid: i32,
    pub uid: i32,
    pub comm: [u8; 50],
}

pub const FORMAT_DECIMAL: &str = "%d";
pub const FORMAT_STRING: &str = "%s";

pub fn now(rx: mpsc::Receiver<Message>) {
    let mut messages: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();

    // checking messages
    for message in rx.iter() {
        match message.command_type {
            message::TYPE_CONTEXT => {
                load_context(&mut messages, message.command_content);
            },
            message::TYPE_SESSION_ALL => {
                for (key, value) in &messages {
                    println!("source : {}", key);
                    for meta in value {
                        println!("{:#?}", meta);
                    }
                }
            },
            message::TYPE_SESSION_REQ => {
                let request = match message::decode::<SessionReq>(message.command_content) {
                    Ok(req) => req,
                    Err(e) => {
                        println!("ERR {}", e);
                        return;
                    }
                };

                let value = request.value;

                for (_, elem) in &messages {
                    for meta in elem {
                        if meta.clone().values().any(|val| val.contains(&value)) {
                            println!("{:#?}", meta);
                        }
                    }
                }
            },

            _ => {}
        }
    }
}

pub fn load_context(list: &mut HashMap<String, Vec<HashMap<String, String>>>, data: Value) {
    let ctx: LoadContext = match serde_json::from_value(data) {
        Ok(data) => data,
        Err(e) => {
            println!("ERR {}", e.to_string());
            return;
        }
    };

    let list_key = format!("{}_{}", ctx.probe, ctx.hook);
    if !list.contains_key(&list_key) {
        list.insert(list_key.clone(), vec![]);
    }

    let mut data: HashMap<String, String> = HashMap::new();
    let size = std::mem::size_of::<Event>();
    let event: Event = unsafe { ptr::read_unaligned(ctx.data.as_ptr() as *const Event) };
    //println!(
    //    "pid={}, comm={}, hook={}, data_len={}",
    //    event.pid,
    //    String::from_utf8_lossy(&event.comm).trim_matches(char::from(0)),
    //    ctx.hook,
    //    ctx.data.len()
    //);

    data.insert("pid".to_string(), format!("{}", event.pid));
    data.insert(
        "comm".to_string(),
        String::from_utf8_lossy(&event.comm).trim_matches(char::from(0)).to_string(),
    );

    let mut rest = ctx.data.clone();
    let _ = rest.drain(0..size);

    for arg in ctx.args {
        if !arg.contains(":") && rest.len() > 0 {
            continue;
        }

        let elem: Vec<&str> = arg.split(":").collect();
        let value = elem[0];
        let format = elem[1].to_lowercase();

        match format.as_str() {
            FORMAT_DECIMAL => {
                let argument: i32 = unsafe { ptr::read_unaligned(rest.as_ptr() as *const i32) };
                rest.drain(0..std::mem::size_of::<i32>());
      //          println!("{}={}", value, argument);
                data.insert(value.to_string(), format!("{}", argument));
            }
            FORMAT_STRING => {
                let mut null = false;
                let mut count: i32 = 0;
                let mut argument: Vec<u8> = vec![];

                for b in rest.clone() {
                    if !null {
                        if b == 00u8 {
                            null = true;
                            count = count + 1;
                            continue;
                        }
                        argument.push(b);
                    } else if b != 00u8 {
                        break;
                    }
                    count = count + 1;
                }
        //        println!("{}={}", value, String::from_utf8_lossy(&argument));
                data.insert(value.to_string(), String::from_utf8_lossy(&argument).to_string());
                rest.drain(0..count as usize);
            }
            _ => continue,
        }
    }

    if let Some(list) = list.get_mut(&list_key) {
        list.push(data);
    }
}
