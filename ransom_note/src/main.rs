use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool 
{
    let mut hash = HashMap::new();

    for x in magazine.bytes()
    {
        let entry = hash.entry(x).or_insert(0);
        *entry += 1;
    }

    for x in ransom_note.bytes()
    {
        if !hash.contains_key(&x) || hash.get(&x) == Some(&0)
        {
            return false;
        }

        let entry = hash.entry(x).or_default();
        *entry -= 1;
    }

    true
}

fn main() 
{
    println!("CAN IT BE CONSTRUCTED? {}", can_construct("HALLOWEEN".to_string(), "BALLS".to_string()));
    println!("CAN IT BE CONSTRUCTED? {}", can_construct("test".to_string(), "test".to_string()));
}
