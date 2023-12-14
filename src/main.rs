mod numbers;
mod variables;
mod char_bool_unit;
mod statements;
mod functions;
mod ownership;
mod borrowing;
mod string;
mod array;

fn main() {
    variables::variables_ex();
    println!();
    numbers::numbers_ex();
    println!();
    char_bool_unit::char_bool_unit_ex();
    println!();
    statements::statements_ex();
    println!();
    functions::functions_ex();
    println!();
    ownership::ownership_ex();
    println!();
    borrowing::borrowing_ex();
    println!();
    string::string_ex();
    println!();
    array::array_ex();
}
