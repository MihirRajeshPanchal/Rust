fn
main(){
    const PI:f32 = 3.14;
    println!("Constant value of pi is {}",PI);
}

// Constants VS variables
// Constants are declared using the const keyword while variables are declared using the let keyword.

// A variable declaration can optionally have a data type whereas a constant declaration must specify the data type. This means const USER_LIMIT=100 will result in an error.

// A variable declared using the let keyword is by default immutable. However, you have an option to mutate it using the mut keyword. Constants are immutable.

// Constants can be set only to a constant expression and not to the result of a function call or any other value that will be computed at runtime.

// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.