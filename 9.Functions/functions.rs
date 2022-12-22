fn main(){

    say_hello();
    say_something("My Name is Mihir".to_string());
    println!("Pi Value is : {}",get_pi());
}

fn say_hello(){
    println!("Hello Mihir");
}

fn say_something(something: String){
    println!("{}",something);
}

fn get_pi()->f64 {
    22.0/7.0
 }