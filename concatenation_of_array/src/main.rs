//WORST MEMORY USAGE
pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> 
{
    let mut new_nums = nums.clone();
    
    for x in nums
    {
        new_nums.push(x);
    }
    new_nums
}
//LAZY WAY BUT SECOND BEST MEMORY USAGE
pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> 
{
    nums.repeat(2)
}

//BEST
pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> 
{
    nums.reserve(nums.len());
    nums.extend_from_within(..);
    nums
}
fn main() 
{
    let mut x = vec![1,2,3];

    x = get_concatenation(x);

    for val in x
    {
        print!("{val} ");
    }
}
