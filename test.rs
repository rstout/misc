// A simplification of what I'm trying to do                                    
                                                                                
// I'm looking for help to fill in the "???" (line 25) or make this code        
// feasible in another way                                                      
                                                                                
// Background: The expected use case is for the ~Iterator<u8> to come from      
// turning a file into a byte stream (file.bytes()). However, to test this,     
// I'd rather not construct ~Iterator<u8>'s from files, but rather construct    
// them from something like a ~[u8]. See the mocked up test mod at the bottom.  
struct BufferedReader<T> {                                                         
    input: ~Iterator<T>,                                                       
    buf: ~[T]                                                                  
}                                                                               
                                                                                
impl BufferedReader {                                                           
    // Reads from self.input and if not None, appends to self.buf               
    fn next(&mut self) -> Option<T> { /* impl */ }                             
                                                                                
    // Clears self.buf                                                          
    fn clear_buf(&mut self) { /* impl */ }                                      
}                                                                               
                                                                                
mod test {                                                                      
    fn make_buffered_reader<T>(buf: ~[T]) -> BufferedReader<T> {                     
        // ???                                                                  
    }                                                                           
                                                                                
    fn test_clear_buf() {                                                       
        let buf = ~[30u8, 31u8];                                                
        let buffered_reader = make_buffered_reader(buf);                        
        // Do stuff to 'buffered_reader' to test it                             
    }                                                                           
} 
