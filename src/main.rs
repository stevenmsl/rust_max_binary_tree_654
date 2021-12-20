use rust_max_binary_tree_654::Solution;

fn main() {
    let nums = Solution::test_fixture_2();
    println!("{:?}", Solution::construct_maximum_binary_tree(nums));
}
