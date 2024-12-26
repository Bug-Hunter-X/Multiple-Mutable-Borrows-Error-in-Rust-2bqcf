fn main() {
    let mut x = 5;
    { //Using a block to limit the scope of y
        let y = &mut x; 
        *y += 1;
    }
    { //Using a block to limit the scope of z
        let z = &mut x; 
        *z += 1;
    } 
    println!("x = {}", x);
}
//Alternative solution: Cloning the variable.
//fn main() {
//    let mut x = 5;
//    let mut y = x;
//    let mut z = x;
//    y += 1;
//    z += 1;
//    x = y + z - 10;
//    println!("x = {}", x);
//}