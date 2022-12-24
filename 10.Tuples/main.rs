fn main(){
    let tuple:(i32,i32,i32)=(-4,0,5);
    println!("{:?}",tuple);

    let b:(i32,bool,f64) = (110,true,10.9);
    print(b);
}

 fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
 }