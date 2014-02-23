// Rust 0.9

use std::io::mem::BufReader;                                                
use std::io::extensions::ByteIterator;

fn get_iter() -> ~Iterator<u8> {                                            
    let buf = ~[30u8, 31u8];                                                
    let mut buf_reader = ~BufReader::new(buf);                  
    let byte_iter = ~ByteIterator::new(buf_reader);                         
    let iter: ~Iterator<u8> = byte_iter as ~Iterator<u8>; // error                 
    iter                                                                  
}       

/*
The above fails to compile with error:

scanner.rs:118:35: 118:44 error: cannot pack type `~std::io::extensions::ByteIterator<,std::io::mem::BufReader<>>`, which does not fulfill `Send`, as a trait bounded by Send
scanner.rs:118         let iter: ~Iterator<u8> = byte_iter as ~Iterator<u8>;
                                                 ^~~~~~~~~
error: aborting due to previous error

However, the doc for ByteIterator (http://static.rust-lang.org/doc/0.9/std/io/extensions/struct.ByteIterator.html)
says that ByteIterator implements Iterator<u8>. So why is rustc complaining about not implementing Send? 
(This is on rust 0.9 btw)
*/
