pub fn run(){
    println!("Hello World but you're gay, also from file");
    println!("And now, behold, an integer:");
    println!("Number: {}",123);
    println!("{} is {}","Alicia","awesome");
    // index for arguments
    println!("these are {1} {0} and they are cool","arguments","positional");
    // named arguments
    println!("{name} likes {genre}",name="Amy",genre="RPGs");
    // placeholder traits 
    println!("Bin: {:b} Hex: {:x} Octal: {:o}",20,20,20);
    // debug trait placeholder
    println!("{:?}",(10,true,false,"hi"));
    // math stuff
    println!("2+2={} minus 1 quick maths",2+2);
}