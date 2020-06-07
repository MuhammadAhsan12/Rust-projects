fn main()
{
    let x = 35;
    println!(" x = {}",x);
    let x = x*5;
    println!(" x = {}",x);
    let x:f64 = 6.1;
    println!(" x = {}",x);

    let x:i32 = -11;
    println!(" x = {}",x);

    let mut age = 38;
    println!(" my age is : {}",age);

    age = 10;
    println!(" my age is : {}",age);
    println!(" {},{}'{},{},{},{}",x,x,x,x,age,age);

    println!("Max i32 : {}",std::i32::MAX);
    println!("Max i64 : {}",std::i64::MAX);
    println!("Max f64 : {}",std::f64::MAX);

    let x=1;
    let y=2.5;
    let z=45;
    let face='\u{1F600}';
    let is_active: bool = true;
    let is_greater: bool = 10<5;
    println!("{:?}",(x,y,z, is_active, is_greater,face));

   let mut temperature = 35;
   println!("initial Temperature : {}",temperature);
    temperature = 38;
    println!("Final Temperature : {}",temperature);

    
    let a=1;
    println!("{}",a);
}