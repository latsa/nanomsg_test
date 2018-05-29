extern crate libc;
extern crate nanomsg;

use std::env;
use std::io::{Read, Write};
use nanomsg::{Socket, Protocol, PollFd, PollRequest, PollInOut};

fn puller(url: &str) {
   let mut input = Socket::new(Protocol::Pull).unwrap();
   input.bind(url).unwrap();
   println!("Puller listening on '{}'.", url);

   let mut pollfd_vec: Vec<PollFd> = vec![input.new_pollfd(PollInOut::In)];
   let mut poll_req = PollRequest::new(&mut pollfd_vec[..]);
   let timeout = 10000;
   match Socket::poll(&mut poll_req, timeout) {
      Ok(_) => {
         let mut msg = String::new();
         match input.read_to_string(&mut msg) {
            Ok(_) => {
               println!("Puller got a message: '{}'.", &*msg);
               msg.clear();
            }, Err(err) => {
               println!("Puller failed '{}'.", err);
               exit(err.kind() as i32);
            }
         }
      }, Err(err) => {
         println!("Puller failed '{}'.", err);
         exit(err as i32);
      }
   }

}


fn pusher(url: &str) {
   let mut output = Socket::new(Protocol::Push).unwrap();
   let mut endpoint = output.connect(url).unwrap();


   let msg = format!("Hello");
   let msg_bytes = msg.as_bytes();
   match output.write_all(msg_bytes) {
      Ok(_) => {
             println!("Simon sez {}.", &*msg);
      }, Err(err) => {
            println!("Pusher failed '{}'.", err);
            exit(err.kind() as i32);
      }
   }

   match endpoint.shutdown() {
      Ok(_) => println!("Simon sez Bye"),
      Err(err) =>  {
         println!("{}", err);
         exit(err as i32);
      }
   }
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

fn env_is_set(envvar:&str) -> bool {
    match env::var(envvar) {
        Ok(_) => true,
        _ => false
    }
}

fn main() {
   let url = "ipc:///tmp/pipeline1.ipc".as_ref();
   let args: Vec<String> = env::args().collect();

   let program_name = &args[0];
   if args.len() == 1 {

     if !env_is_set("mode") {
         usage(program_name);
     }

     let mode: &str = &env::var("mode").unwrap();

     match mode {
       "push" => pusher(url),
       "pull" => puller(url),
       _ => usage(program_name)
     }
   } else if args.len() == 2 {
      let mode = args[1].as_ref();

      match mode {
        "push" => pusher(url),
        "pull" => puller(url),
        _ => usage(program_name)
      }
   } else {
      usage(program_name);
   }
   exit(0);
}
