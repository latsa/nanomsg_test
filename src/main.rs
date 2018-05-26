extern crate libc;
extern crate nanomsg;

use std::env;

fn puller() {
}

fn pusher() {
}

fn exit(code:i32) {
   unsafe { libc::exit(code); };
}

fn usage(program_name : &String) {
   println!("");
   println!("Usage: {} [push|pull]",program_name);
   println!("");
   exit(1);
}

fn main() {
   let args: Vec<String> = env::args().collect();

   let program_name = &args[0];
   if args.len() != 2 {
        usage(program_name);
    }

   let mode = args[1].as_ref();

   match mode {
        "push" => pusher(),
        "pull" => puller(),
        _ => usage(program_name)
    }

   exit(0);
}
