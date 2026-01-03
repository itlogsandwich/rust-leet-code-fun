// pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 
// {
//     let mut total = 0; 
//
//     for acc in accounts
//     {
//         let mut wealth = 0;
//
//         for money in acc
//         {
//             wealth += money;
//         }
//
//         if total < wealth
//         {
//             total = wealth;
//         }
//     }
//
//     total
// }

//FASTEST
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32
{
    accounts.into_iter().map(|acc| acc.into_iter().sum()).max().unwrap()
}

fn main() 
{
    let mut accounts: Vec<Vec<i32>> = Vec::new();

    accounts.push(vec![500,35,60]);
    accounts.push(vec![67,67,67]);
    accounts.push(vec![222,333,444]);

    let max_wealth = maximum_wealth(accounts);

    println!("Total Wealth: {max_wealth}");
}
