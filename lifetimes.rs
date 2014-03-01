// Question: is there any way to fix this function using lifetimes (as the error message would suggest)?                                                       
                                                                                
use std::iter::Map;                                                             
use std::vec::Items;                                                            
                                                                                
fn main () {}                                                                   
                                                                                
fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {                 
    items.map(|&x| x)                                                           
}                                                                               
                                                                                
/*                                                                              
Generates typical lifetimes error message:                                      
                                                                                
lifetimes.rs:9:5: 9:10 error: cannot infer an appropriate lifetime for region i\
n type/impl due to conflicting requirements                                     
lifetimes.rs:9     items.map(|&x| x)                                            
                   ^~~~~                                                        
lifetimes.rs:8:63: 10:2 note: first, the lifetime cannot outlive the anonymous \
lifetime #2 defined on the block at 8:62...                                     
lifetimes.rs:8 fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {  
lifetimes.rs:9     items.map(|&x| x)                                            
lifetimes.rs:10 }                                                               
lifetimes.rs:9:5: 9:22 note: ...so that types are compatible (expected `std::it\
er::Map<,&u8,u8,std::vec::Items<'r,u8>>` but found `std::iter::Map<,&u8,u8,std:\
:vec::Items<,u8>>`)                                                             
lifetimes.rs:9     items.map(|&x| x)                                            
                   ^~~~~~~~~~~~~~~~~                                            
lifetimes.rs:8:63: 10:2 note: but, the lifetime must be valid for the lifetime \
&'r  as defined on the block at 8:62...                                         
lifetimes.rs:8 fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {  
lifetimes.rs:9     items.map(|&x| x)                                            
lifetimes.rs:10 }                                                               
lifetimes.rs:9:5: 9:22 note: ...so that types are compatible (expected `std::it\
er::Map<,&u8,u8,std::vec::Items<'r,u8>>` but found `std::iter::Map<,&u8,u8,std:\
:vec::Items<,u8>>`)                                                             
lifetimes.rs:9     items.map(|&x| x)                                            
                   ^~~~~~~~~~~~~~~~~                                            
error: aborting due to previous error
*/
