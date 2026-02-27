
// slightly less effiecent memory cost
// pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32
// {
//     let mut max = 0;
//     let mut count = 0;
//     nums.iter().for_each(|x| 
//     { 
//         println!("{x}");
//         if *x == 1
//         {
//             count += 1;
//         }
//         else
//         {
//             count = 0;
//         }
//
//         max = std::cmp::max(max,count);
//     });
//     max
// }

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32
{
    let mut max = 0;
    let mut count = 0;

    for x in nums
    {
        if x == 1
        {
            count += 1;
        }
        else
        {
            count = 0;
        }
        max = std::cmp::max(max, count);
    }
    max
}

fn main() 
{
    let max = find_max_consecutive_ones(vec![1,0,0,1,1,0]);

    println!("{max}");
}
