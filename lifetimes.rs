// Question: is there any way to fix this function using lifetimes (as the error message would suggest)?

use std::iter::Map;                                                             
use std::vec::Items;

fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {                             
    items.map(|&x| x)                                                           
} 

/*
Generates typical lifetimes error message:

test2.rs:26:5: 26:10 error: cannot infer an appropriate lifetime for region in type/impl due to conflicting requirements
test2.rs:26     items.map(|&x| x)
                ^~~~~
test2.rs:25:63: 27:2 note: first, the lifetime cannot outlive the lifetime &'r  as defined on the block at 25:62...
test2.rs:25 fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {
test2.rs:26     items.map(|&x| x)
test2.rs:27 }
test2.rs:26:5: 26:22 note: ...so that types are compatible (expected `std::iter::Map<,&u8,u8,std::vec::Items<'r,u8>>` but found `std::iter::Map<,&u8,u8,std::vec::Items<,u8>>`)
test2.rs:26     items.map(|&x| x)
                ^~~~~~~~~~~~~~~~~
test2.rs:25:63: 27:2 note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the block at 25:62...
test2.rs:25 fn f<'r>(items: Items<'r, u8>) -> Map<&u8, u8, Items<'r, u8>> {
test2.rs:26     items.map(|&x| x)
test2.rs:27 }
test2.rs:26:5: 26:22 note: ...so that types are compatible (expected `std::iter::Map<,&u8,u8,std::vec::Items<'r,u8>>` but found `std::iter::Map<,&u8,u8,std::vec::Items<,u8>>`)
test2.rs:26     items.map(|&x| x)
                ^~~~~~~~~~~~~~~~~
error: aborting due to previous error
*/
