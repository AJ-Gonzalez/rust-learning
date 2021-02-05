pub fn run(){
    // unsigned integers u8, u16 etc
    // signed integers i8, i16, i32 ... i128

    let x=5;
    let y=2.5;
    println!("int: {} float: {}",x,y);

    let z: i128 = 2463254823709840923894673465;
    println!("{}",z);

    println!("Max size for i128 is {}",std::i128::MAX);
    println!("Max size for i64 is {}",std::i64::MAX);
    println!("Max size for i32 is {}",std::i32::MAX);
    println!("Max size for i16 is {}",std::i16::MAX);
    println!("Max size for i8 is {}",std::i8::MAX);

    println!("Max size for f64 is {}",std::f64::MAX);
    println!("Max size for f32 is {}",std::f32::MAX);

    println!("Max size for u128 is {}",std::u128::MAX);
    println!("Max size for u64 is {}",std::u64::MAX);
    println!("Max size for u32 is {}",std::u32::MAX);
    println!("Max size for u16 is {}",std::u16::MAX);
    println!("Max size for u8 is {}",std::u8::MAX);

    let is_ok=true;
    println!("{:?}",(x,y,z,is_ok));
  
    let greater = 10 < 5;

    println!("{}",greater);

    let c = 'b';

    println!("{}",c);

    let face = '\u{1F600}';

    println!("{}",face);
}