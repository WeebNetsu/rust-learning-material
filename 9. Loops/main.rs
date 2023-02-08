fn main(){
    let mut num = 0;

    // infinite loop
    loop {
        // ++ is not supported in Rust, have to use +=
        num += 1;

        if num % 2 == 0 {
            continue;
        }

        if num >= 10 {
            break;
        }

        println!("{}" , num);
    }

    // while loop
    while num > 0 {
        num -= 1;

        println!("{}" , num);
    }

    // for range
    for x in 0..10 {
        println!("{}" , x);
    }
}