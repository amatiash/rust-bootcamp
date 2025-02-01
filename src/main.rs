mod comments;
mod constants_and_statics;
mod control_flow;
mod data_types;
mod functions;
mod variables;

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
    constants_and_statics::code();
    constants_and_statics::constants();
    constants_and_statics::statics();
    functions::code();
    functions::definition();
    functions::return_types();
    functions::return_keyword();
    control_flow::code();
    control_flow::if_else();
    control_flow::loop_task();
    control_flow::while_task();
    control_flow::for_task();
    comments::code();
}
