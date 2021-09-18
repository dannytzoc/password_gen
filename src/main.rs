use std::io; //rreading in from standard io
fn main() {
    println!("Welcome to password Generater"); // print line into terminal
    println!("How long do you want the passowrds? ");
    let mut pass_num = String::new(); // make a string
    io::stdin() // read in from standard io
        .read_line(&mut pass_num)
        .unwrap(); // display an error if its wrong
        use rand::Rng; //able to use to generate a random numeb
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~"; //byte string literal constructs a [u8] instead of a string b"..."
        let PASSWORD_LEN: i32 = pass_num.trim().parse::<i32>().unwrap(); //return a string slice with leading and trailing white space removed trim()
        // parses this string into another type and unwrap for error handeling
        let mut rng = rand::thread_rng(); // lazy initialized thread local random number generator
        let password: String = (0..PASSWORD_LEN) //makes a string with a size fo password length
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len()); // goes in the range of 0 to char
            CHARSET[idx] as char //sets it
        })
        .collect();
// collect() can take anything iterable, and turn it into a relevant collection.
    println!("{:?}", password)
//print the password 

}
