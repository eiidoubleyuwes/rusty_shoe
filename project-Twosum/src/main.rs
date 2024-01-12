//Today I'll be solving the popular leetcode problem of two sum
//Problem - Given an array of integers nums and an integer target, 
//return indices of the two numbers such that they add up to target.
//You may assume that each input would have exactly one solution, and you may not use the same element twice.
//You can return the answer in any order.
//Example 1:
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Output: Because nums[0] + nums[1] == 9, we return [0, 1].
//Example 2:
//Input: nums = [3,2,4], target = 6
//Output: [1,2]

//Solution
//We will be using a hashmap to solve this problem
//We will iterate through the array and check if the hashmap contains the difference between the target and the current element
//If it does, we return the current index and the index of the difference
//If it doesn't, we add the current element and its index to the hashmap
//Time Complexity - O(n)
//Space Complexity - O(n)

use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
