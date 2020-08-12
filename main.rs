use std::net::{TcpListener,TcpStream};  //Libraries used
use std::thread;
use std::time;
use std::io::{self,Read,Write};

fn handle_client(mut stream: TcpStream) -> io::Result<()>{   //a function to handle the client
  let mut buf = [0;512];
  for _ in 0..1000 {   //loop
    let bytes_read = stream.read(&mut buf)?;
    match stream.read(&mut buf){   //matching
      Ok(n) => {
        if n==0 {  
          break;   //over if empty
        }
        stream.write(&buf[..bytes_read])?;     //not finished and print it 
        thread::sleep(time::Duration::from_secs(1));    //dalay
      }
      Err(e) => {}
    }
  }
  Ok(())
}

fn main() -> io::Result<()> {   
  let listener = TcpListener::bind("127.0.0.1:8080")?;  //establish listener, use bind function binds IP and port, Return error if an error occurs
  let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();   //create a container to hold the thread handle

  for stream in listener.incoming(){
    let stream = stream.expect("failed");    //data stream
    let handle = thread::spawn(move || {    //create threads for each stream to handle
      handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));  //print the error
    });
    thread_vec.push(handle);    //add the handle to the container
  }

  for handle in thread_vec {
    handle.join().unwrap();   //wait for the thread to end
  }

    Ok(())
  }