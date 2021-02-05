pub fn run(){
    let name ="Alicia";
    // let mut num = 350;
    // let mut varname makes it mutable.
    let num = 426;
    println!("{}'s fave number is {}",name,num);
    // constants
    const EX: i64 = 234;
    println!("{}",EX);
    // assign multiple vars
    let (engine,displacement) = ("SBC",350);
    println!("{} is {} cubic inches",engine,displacement);
}