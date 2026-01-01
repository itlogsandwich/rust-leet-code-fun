pub fn fizz_buzz(n: i32) -> Vec<String> 
{
    let mut vec: Vec<String> = Vec::new();

    for i in 1..=n
    {
        match (i % 3, i % 5)
        {
            (0,0) => vec.push("FizzBuzz".to_string()),
            (0,_) => vec.push("Fizz".to_string()),
            (_,0) => vec.push("Buzz".to_string()),
            _ => vec.push(i.to_string()),
        }
    }
    vec
}

fn main() 
{
    let vec = fizz_buzz(16);

    for x in vec
    {
        println!("{x} ");
    }
}
