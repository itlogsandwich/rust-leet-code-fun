pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> 
{
    for index in (0..digits.len()).rev()
    {
        digits[index] += 1;

        if digits[index] < 10
        {
            return digits;
        }
        
        digits[index] = 0;
    }

    digits.resize(digits.len() + 1, 0);
    digits[0] = 1;
    digits
}
fn main() 
{
    let mut x = vec![1,2,9];

    x = plus_one(x);

    for val in x
    {
        print!("{val}");
    }
}
