pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> 
{
    let mut new_vec = Vec::with_capacity(nums.capacity());
    let mut v = n;

    for x in &nums[..(n as usize)]
    {
        new_vec.push(*x);
        if let Some(val) = nums.get(v as usize)
        {
            new_vec.push(*val);
            v += 1;
        }
    }
    new_vec
}

fn main() 
{
    let vec = vec![1,2,3,4,5,6];
    let bruh = shuffle(vec, 3);

    //1,4,2,5,3,6
    println!("{:?}", bruh);
}
