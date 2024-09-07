fn main() {
    // Describe Value Type
    const DATA_PATH :&str = "this/is/data/path"; // const in Rust have to add type describe
    let data = 1; // var default is immutable
    let mut variable_data = 1; // add mut to be a mutable var
    let flag:bool = true;

    let flag = 'A'; // create a new flag and shadowing the old flag
    let _y = "a"; // add '_' in front of var name can ignore the compiler warn of "unused variable: y"
}