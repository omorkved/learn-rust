// Loosely following The Rust Programming Language book, comments mine
fn main() {

    /* Let's learn ownership */
    let s = String::from("hello");  

    takes_ownership(s);             
    
    /* This call wouldn't work a second time:
     * takes_ownership(s);  
     *
     * because ownership of s transferred to child fcn
     * So, though s still points to the same String on
     * the heap, the compiler flags it as moved
     */ 

    let x = 5;                      

    makes_copy(x);

    /* x is still in scope, as i32 has the Copy trait.
     * x's value in main, stored on Stack --> copied into
     * _x's value in makes_copy, also stored on Stack
     */ 
    makes_copy_also(x);

    /*Let's learn Referencing */
    let greeting = String::from("Hello");
    /* cons makes me feel like I'm writing Standard ML and 
     * Elm again. Kinda miss functional programming :') 
     */

    does_not_get_ownership(&greeting);
    /* We've retained ownership, so we can refer to greeting again */
    println!("{} again", greeting);

} //drops greeting, drops x


fn takes_ownership(s : String) -> String { //This feels like Elm and C had a baby :)
    s
} //drops s

fn makes_copy(_x : i32){} //no need to specify returns Null

fn makes_copy_also(_x : i32){
    return; //can use return keyword, but not necessary
}

fn does_not_get_ownership(str : &String){
    println!("{}", str);
} // does not drop str