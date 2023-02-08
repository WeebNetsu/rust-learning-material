fn main(){
    let age = 18;

    if age > 17 {
        /* 
            You of course have all of these for comparisons:
            < > <= >= == !=
        */
        
        println!("You may enter the bar");
    } else {
        println!("You may not enter the bar");
    }

    let birthday = true;

    if birthday && age > 17 {
        /* 
            You of course have all these for mixing and matching:
            && || !
        */
        
        println!("You may enter and get 1 free drink!");
    } else if age > 17 {
        println!("You may enter the bar");
    } else {
        println!("You may not enter the bar");
    }

    // shorthand if (single line if statement)
    let name = if birthday { "Nicky" } else { "Nick" };
    println!("{}", name);
}