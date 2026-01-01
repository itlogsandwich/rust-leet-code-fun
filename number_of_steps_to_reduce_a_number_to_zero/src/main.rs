pub fn number_of_steps(mut num: i32) -> i32 
{
    let mut count = 0;
    while num > 0
    {
        num = match num % 2
        {
            0 => num / 2,
            _ => num - 1,
        }; 
        count += 1;
    }
    
    count
}
fn main() 
{
    let num = number_of_steps(8);
    println!("Steps {num}");
}
