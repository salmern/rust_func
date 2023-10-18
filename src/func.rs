pub fn print_number_to(num: u32){
    for n in 1..num{
        print!("{}", n);
          
    if is_even(n){
        println!(" The Number is Even!");
    }else{
        println!(" The Number is Odd!");
    }
    }
}

pub fn is_even(num: u32)-> bool{
    return num % 2==0;
}

