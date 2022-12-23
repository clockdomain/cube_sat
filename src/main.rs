#![allow(unused_variables)]

use std::rc::Rc; 

#[derive(Debug)]
struct Message {
    to : u64,
    payload : String,
}



#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>
}

impl Mailbox {
    fn new() -> Self {
        Self { messages : vec![] }
    }
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn deliver (&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }

}

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}


enum StatusMessage {
    Ok,
}

fn check_status(sat_id : &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

struct GroundStation {
}

impl GroundStation {
    fn send(&self, mailbox : &mut Mailbox,
            message: Message) {
        mailbox.post(message);
    }
    fn connect (&self, sat_id : u64) -> CubeSat {
        CubeSat {
            id: sat_id,
        }
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0,1,2,3]
}

fn main() {
    let gs = Rc::new (GroundStation {});
    let mut mailbox = Mailbox::new();
  
 
    let sat_ids = fetch_sat_ids();
  


 
    for sat_id in sat_ids {
        let cube_sat = gs.connect(sat_id);
        let status = check_status(&cube_sat);
        match status  {
            StatusMessage::Ok => { 
                let message = Message { to: cube_sat.id, payload : "Hello!".to_string() };
                gs.send(&mut mailbox, message); 
                if let Some(msg) = cube_sat.receive(&mut mailbox) {
                    println! ("received: {} {}", msg.to, msg.payload);
                }
            },
            _ => { println!("Satelite down"); },
        }
    }

}
