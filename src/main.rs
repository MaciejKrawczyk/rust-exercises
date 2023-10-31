#![allow(unused)]

use std::io;
use rand::{random, Rng};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {

    // println!("What is your name? ");
    //
    // let mut name: String = String::new();
    //
    // let greeting: &str = "Nice to meet you";
    //
    // io::stdin().read_line(&mut name)
    //     .expect("Didn't receive input"); // ok | error
    //
    // println!("Hello {}! {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    //
    // let age: &str = "47"; // string with "" and characters with ''
    //
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    //
    // age = age + 1;
    //
    // println!("I'm {} and I want ${}", age, ONE_MIL)


    // Unsigned integer : u8, u16, u32, u62, u128, usize...
    // Signed integer : ... the same

    // println!("Max u32: {}", u32::MAX) // checking the max that can be stored in this datatype

    // let is_true: bool = true; // false
    // let my_grade = 'A';

    // let num_1: f32 = 1.111111111111111;
    // println!("f32: {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64: {}", num_2 + 0.111111111111111);  // check the precision

    // let num_3: u32 = 5;
    // let num_4: u32 = 4;

    // println!("5+4={}", num_3 + num_4);

    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("{}", random_num);

    // let age = 8;
    //
    // if (age >= 1) && (age <= 18) {
    //     println!("important birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("other important birthday");
    // } else if age >= 65 {
    //     println!("important....")
    // } else {
    //     println!("not important")
    // }

    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 {
    //     true
    // } else {
    //     false
    // };
    //
    // println!("Can vote: {}", can_vote);

    // let age2 = 8;
    //
    // match age2 {
    //     1..=18 => println!("important borthday"),
    //     21 | 50 => println!("important birtfasdtd"),
    //     65..=i32::MAX => println!("important birthday"),
    //     _ => println!("not important"),
    // };

    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("can't vote"),
    //     Ordering::Greater => println!("can vote"),
    //     Ordering::Equal => println!("you gained the permission to vote")
    // };

    // let arr_1 = [1,2,3,4,5,6,7,8,9];
    // println!("1st: {}", arr_1[0]);
    // println!("length: {}",arr_1.len());
    //
    // let mut loop_idx = 0;
    // loop {
    //     if arr_1[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_1[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("val: {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    // let arr_2 = [1,2,3,4,5,6,7,8,9];
    // let mut loop_idx = 0;
    // while loop_idx < arr_2.len() {
    //     println!("arr: {}", arr_2[loop_idx]);
    //     loop_idx+=1;
    // }
    //
    // for val in arr_2.iter() {
    //     println!("Val: {}", val);
    // }

    // let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    //
    // println!("name: {}", my_tuple.1);
    // let(v1, v2, v3) = my_tuple;
    // println!("age: {}", v1);

    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }
    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // let st3 = String::from("x r t b h k k a m c");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }
    // let st4: &str = "Random string";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);
    //
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("String length: {}", st6.len());
    // st5.clear();
    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7;
    //
    // for char in st8.bytes() {
    //     println!("{}", char);
    // }



}
