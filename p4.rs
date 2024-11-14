fn main()
{
    let mut num : u32;
    let mut largest_palindrome: u32 = 0;


    for i in 100..1000
    {
        for j in 100..1000
        {
            num = i*j;
            let num_str : String = num.to_string();
            let num_reversed_str : String = num_str.chars().rev().collect();

            if num_str == num_reversed_str && num > largest_palindrome 
            {
                largest_palindrome = num;
            }
        }
    }
    
    println!("largest palindrome: {}", largest_palindrome);

}

