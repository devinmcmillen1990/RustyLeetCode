use super::{
    add_two_numbers_iteratively_loop_unroll::add_two_nums_iteratively_loop_unroll,
    add_two_numbers_tests_helpers::{assert_linked_list_eq, get_test_cases, vec_to_list},
};

#[test]
fn test_add_two_nums_iteratively() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let l1 = vec_to_list(case.l1);
        let l2 = vec_to_list(case.l2);
        let result = add_two_nums_iteratively_loop_unroll(l1, l2);
        assert_linked_list_eq(result, case.expected, index);
    }
}
