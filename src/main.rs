mod variables;
mod data_types;

fn main() {
    variables::code();
    variables::fix_variable_definition();
    variables::fix_variable_definition_2();
    variables::fix_code_with_shadowing();
    data_types::code();
    data_types::boolean();
    data_types::unsigned_int();
    data_types::signed_int();
    data_types::floating_point_numbers();
    data_types::char();
    data_types::string_types();
    data_types::arrays();
    data_types::tuples();
    data_types::type_aliasing();
}
