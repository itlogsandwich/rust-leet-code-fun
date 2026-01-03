// pub fn running_sum(nums: Vec<i32>) -> Vec<i32> 
// {
//     let mut sum = 0;
//     let mut new_nums: Vec<i32> = Vec::new();
//
//     for x in nums
//     {
//         sum += x;
//         new_nums.push(sum);
//     }
//
//     new_nums
// }

//FAST BUT SLOWER IN MEMORY
// pub fn running_sum(nums: Vec<i32>) -> Vec<i32> 
//    {
//        nums.into_iter().scan(0, |sum, x| {*sum += x; Some(*sum)} ).collect()
//    }

//FASTEST
pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> 
{
    for x in 1..nums.len(){nums[x] += nums[x - 1];}
    nums
}
fn main() 
{
    let x = vec![1,2,3];

    print!("1st Vec: ");
    for val in &x
    {
        print!("{val} ");
    }

    println!("");

    let sum = running_sum(x);

    print!("2nd Vec: ");
    for xal in sum
    {
        print!("{xal} ");
    }
}
