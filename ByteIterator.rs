use std::io::BufReader;                                                         
use std::io::extensions::Bytes;                                                 
         
// This doesn't work, but is there any way to turn a ~[u8] into a ~Iterator<u8>? (Maybe using 
// Lifetimes, Gc or something else?)
// I believe the issue is with 'buf_reader', in that it gets cleaned up after get_iter() returns
fn get_iter(buf: ~[u8]) -> ~Iterator<u8> {                                      
    let mut buf_reader = ~BufReader::new(buf);                                  
    let byte_iter = ~Bytes::new(buf_reader);                                     
    let iter = byte_iter as ~Iterator<u8>;                                      
    iter                                                                        
}                                                                               

/*
Comipler error message:

test.rs:8:16: 8:25 error: cannot pack type `~std::io::extensions::Bytes<,std::io::mem::BufReader<>>`, which does not fulfill `Send`, as a trait bounded by Send
test.rs:8     let iter = byte_iter as ~Iterator<u8>;
                         ^~~~~~~~~
error: aborting due to previous error
*/

