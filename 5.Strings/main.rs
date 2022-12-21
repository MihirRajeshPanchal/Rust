fn 
main(){
    //String Literal
    let name:&str="Mihir";
    println!("String Literal : {}",name);

    let surname:&'static str = "Panchal";
    println!("Static String Literal : {}",surname);

    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("Mihir Panchal");
    println!("length is {}",content_string.len());


    let mut z = String::new();
    z.push_str("     Hello");
    println!("{}",z);
    z.push_str(" World   ");
    println!("{}",z);

    println!("After Trim : {}",z.trim());

    let name ="Mihir";
    println!("{}",name.to_string());

    println!("{}",z.replace("Hello","Hi").trim());

    let names="Mihir,Chomu,Kuber";
    for i in names.split(","){
        println!("{}",i);
    }

    let concat_string = name.to_string() + &z;
    println!("{}",concat_string);
}