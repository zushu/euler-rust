fn main()
{
    println!("{:?}", factors(600851475143));

}

fn factors(mut n: u64) -> Vec<u64>
{
    let mut z : u64 = 2;
    let mut output = Vec::new();

    while z*z <= n
    {
        if n % z == 0
        {
            output.push(z);
            n = n/z;
        }
        else { z = z + 1; }

    }
    if n > 1 { output.push(n); }

    return output;
}
