fn main()
{
    let mut n : u64;
    let mut counter : u64 = 1;
    let mut sum : u64 = 0;

    loop
    {
        n = fibonacci(counter);
        counter = counter+1;
        if n > 4000000 { break; }

        else if n % 2 == 0 { sum += n; } 
    }

    println!("{}", sum);
}

fn fibonacci(n: u64) -> u64
{
    if n <= 0       { return 0; }
    else if n == 1  { return 1; }
    else if n == 2  { return 2; }
    else            { return fibonacci(n-1) + fibonacci(n-2); } 
}
