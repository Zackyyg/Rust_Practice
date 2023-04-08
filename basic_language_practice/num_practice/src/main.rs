use num_bigint::BigUint;
use num_traits::Num;

fn main() {
    big_int_practce();
    println!("Hello, world!");
}


fn big_int_practce(){
    let x = BigUint::from_str_radix("100000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap();
    let big_num = BigUint::from_str_radix("10000000000000000000000000000000000000000000000000", 10).unwrap();
    let y = &x % &big_num;
    let divsion = &x / &big_num;
    println!("{}", y);
    println!("{}", divsion);
}
