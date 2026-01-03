use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
{
    let mut hash = HashMap::new();
    
    for (index, &num) in nums.iter().enumerate()
    {
        let difference = target - num;  

        if let Some(&val) = hash.get(&difference)
        {
            return vec![val as i32, index as i32];
        }

        hash.insert(num, index);
    }

    vec![]
}
fn main() 
{
    let nums = two_sum(vec![1,3,5,6,7], 10);
    
    for x in nums
    {
        print!("Index: {x}, ");
    }
}
