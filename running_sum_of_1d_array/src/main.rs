pub fn running_sum(nums: Vec<i32>) -> Vec<i32> 
    {
        let mut sum = 0;
        let mut new_nums: Vec<i32> = Vec::new();

        for x in nums
        {
            sum += x;
            new_nums.push(sum);
        }

        new_nums
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
