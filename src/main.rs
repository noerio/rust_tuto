mod numbers;
mod variables;
mod char_bool_unit;
mod statements;
mod functions;
mod ownership;

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
}
