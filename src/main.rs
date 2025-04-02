mod borrowing;
mod box_smart_pointer;
mod comments;
mod concrete_lifetimes;
mod constants_and_statics;
mod control_flow;
mod data_types;
mod deriving_traits;
mod enums_and_matching;
mod functions;
mod generic_lifetimes;
mod generics;
mod implementation_blocks;
mod modules;
mod option_and_result;
mod ownership;
mod slices;
mod structs;
mod structs_and_lifetime_elision;
mod supertraits;
mod trait_objects;
mod traits;
mod tuple_structs;
mod unit_tests;
mod variables;
mod vectors;
mod rc_smart_pointer;

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
    ownership::code();
    ownership::moving_on_assignment();
    ownership::moving_on_assignment_2();
    ownership::moving_into_function();
    ownership::moving_out_of_function();
    borrowing::code();
    borrowing::immutable_references();
    borrowing::mutable_references_1();
    borrowing::mutable_references_2();
    borrowing::passing_by_reference();
    slices::code();
    slices::string_slice();
    slices::array_slice();
    structs::code();
    structs::struct_definition();
    structs::mutating_structs();
    structs::structs_and_functions();
    implementation_blocks::code();
    implementation_blocks::methods();
    implementation_blocks::associated_functions();
    tuple_structs::code();
    tuple_structs::definition();
    enums_and_matching::code();
    enums_and_matching::definition_1();
    enums_and_matching::definition_2();
    enums_and_matching::exhaustive_requirement();
    enums_and_matching::match_expression();
    enums_and_matching::matching_on_enums_1();
    enums_and_matching::matching_on_enums_2();
    option_and_result::code();
    option_and_result::matching_option();
    option_and_result::matching_option_2();
    option_and_result::if_let();
    option_and_result::matching_result();
    option_and_result::returning_result();
    vectors::code();
    vectors::pushing();
    vectors::removing();
    vectors::fetching();
    vectors::iterating();
    vectors::iterating_and_mutation();
    modules::visibility();
    modules::bringing_item_into_scope();
    modules::multi_file_projects_1();
    modules::multi_file_projects_2();
    modules::re_exporting();
    generics::code();
    generics::generic_type_example();
    generics::defining_generic_types_1();
    generics::defining_generic_types_2();
    generics::implementation_blocks();
    generics::generic_functions();
    traits::code();
    traits::implementing_traits_1();
    traits::implementing_traits_2();
    traits::default_implementations();
    traits::overriding();
    traits::specifying_trait_bounds_1();
    traits::specifying_trait_bounds_2();
    traits::specifying_trait_bounds_3();
    traits::multiple_trait_bounds_1();
    traits::multiple_trait_bounds_2();
    traits::returning_trait_bounds();
    supertraits::code();
    supertraits::implementing_supertraits();
    supertraits::multiple_supertraits();
    trait_objects::code();
    trait_objects::returning_and_passing_to_functions();
    trait_objects::trait_objects_vectors();
    deriving_traits::code();
    deriving_traits::deriving_on_structs();
    deriving_traits::deriving_on_enums();
    concrete_lifetimes::code();
    concrete_lifetimes::lifetimes_of_owned_values();
    concrete_lifetimes::dangling_references();
    concrete_lifetimes::non_lexical_lifetimes();
    generic_lifetimes::code();
    generic_lifetimes::helping_the_borrow_checker();
    generic_lifetimes::complying_with_the_borrow_checker();
    structs_and_lifetime_elision::code();
    structs_and_lifetime_elision::lifetimes_in_structs();
    structs_and_lifetime_elision::lifetime_elision();
    box_smart_pointer::code();
    box_smart_pointer::creation();
    box_smart_pointer::recursive_types();
    rc_smart_pointer::code();
    rc_smart_pointer::counting_refs_1();
    rc_smart_pointer::counting_refs_2();

}
