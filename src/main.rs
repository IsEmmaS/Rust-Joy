fn add(a:i32, b:i32)->i32{
    // rust function var type is essential
    a + b
}

fn return_null(a:i32){
    // This fn return a "()"
    // or writen as
    // fn return_null(a:i32)->()
    // is also good
}

fn function_diverge()->!{
    panic!("Never return!")
    // diverge function never return anything
    // always used as error expect
}


fn main() {
    // Describe Value Type
    const DATA_PATH :&str = "this/is/data/path"; // const in Rust have to add type describe
    let data = 1; // var default is immutable
    let mut variable_data = 1; // add mut to be a mutable var
    let flag:bool = true;

    let flag = 'A'; // create a new flag and shadowing the old flag
    let _y = "a"; // add '_' in front of var name can ignore the compiler warn of "unused variable: y"
    let a = return_null(data);
    println!("{:?}", a) // Add really return "()".
}