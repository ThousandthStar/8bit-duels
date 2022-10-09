use std::{sync::{Arc, Mutex}, collections::VecDeque, net::TcpStream, thread, io::Write, panic};


pub(crate) fn spawn_output_thread(queue_out_ref: Arc<Mutex<VecDeque<String>>>, mut stream: TcpStream){
    thread::spawn(move || {
        loop{
            let mut guard = queue_out_ref.lock().unwrap();
            if guard.is_empty() {drop(guard); continue;}

            if stream.write(guard.pop_front().unwrap().as_bytes()).is_err(){
                panic!("Error Writing to the server!");
            }
            drop(guard);
        }
    });
}