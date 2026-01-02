// use std::collections::HashMap;

//ATTEMPT 3 - FASTEST
pub fn repeated_n_times(nums: Vec<i32>) -> i32
{
    for x in 0..nums.len()
    {
        if nums[x] == nums[(x + 1) % nums.len()]|| nums[x] == nums[(x + 2) % nums.len()]
        {
            return nums[x];
        }
    }
    unreachable!()
}
//ATTEMPT 2 - FASTER
// pub fn repeated_n_times(nums: Vec<i32>) -> i32 
// {
//     let mut hash = HashMap::with_capacity(nums.len() / 2 + 1);
//
//     for x in nums
//     {
//         let entry = hash.entry(x).or_insert(0);
//
//         *entry += 1;
//
//         if *entry > 1 
//         {
//             return x;
//         }
//     }
// }

//ATTEMPT 1 - WORST OF THEM ALL! 
// pub fn repeated_n_times(nums: Vec<i32>) -> i32 
// {
//     let mut count = 0;
//     let mut target = 0;
//
//     let mut hash = HashMap::new();
//
//     for x in nums
//     {
//         hash.entry(x).and_modify(|val| *val += 1).or_insert(1);
//     }
//
//     for (index, num) in &hash
//     {
//         if *num > count
//         {
//             count = *num;
//             target = *index;
//         }
//     }
//
//     target
//
// }
fn main() 
{
    println!("VALUE BEING REPEATED: {}", repeated_n_times(vec![6,6,2,1,3,4]));
}
