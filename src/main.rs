use std::time::{Duration, Instant};
use string_manip_benchmark::*;
use string_manip_benchmark::str_concat;

fn main() {
    let loops: usize = 10;
    let total_words_u: usize = 1000; // How many times to repeat 'test_str' before and after 'find_str'

    let test_str: &str = "tést "; // é has 2 bytes 
    let find_str: &str = "find me "; // Text tu put in the middle

    // String lengths
    println!("Byte length test_str: {}", test_str.len());

    let find_str_len_u: usize = find_str.len();
    let find_str_len_i: isize = find_str_len_u as isize;
    let test_str_len_u: usize = test_str.chars().count();

    let s_capacity: usize = test_str_len_u * total_words_u * 2 + find_str_len_u;
    let mut s: String = String::with_capacity(s_capacity); // In bytes

    let find_str_pos_u: usize = test_str_len_u * total_words_u;
    let find_str_pos_i: isize = find_str_pos_u as isize;

    s += &test_str.repeat(total_words_u);
    s.push_str(&find_str);
    s += &test_str.repeat(total_words_u);
    println!("Capacity check: {} = {}\n", s_capacity, s.capacity());

    let mut start_time: Instant;
    let mut elapsed_time: Duration;

    // -------------------------------------------------------------------------

    println!("indexof,indexof2,indexof3,indexof4,indexof5,indexof6,indexof7");

    for _ in 0..loops {
        start_time = Instant::now();
        let _result: usize = indexof(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof2(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof3(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof4(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof5(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof6(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: usize = indexof7(&s, &find_str, 0).unwrap();
        elapsed_time = start_time.elapsed();
        print!("{:?}", elapsed_time.as_nanos());

        println!("");
    }

    // -------------------------------------------------------------------------

    println!("");
    println!("chars,substr,substr2,substr3,substr4");

    for _ in 0..loops {
        start_time = Instant::now();
        let _result: String = s
            .chars()
            .skip(find_str_pos_u)
            .take(find_str_len_u)
            .collect::<String>();
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = substr(&s, find_str_pos_i, find_str_len_i);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = substr2(&s, find_str_pos_i, find_str_len_i);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = substr3(&s, find_str_pos_i, find_str_len_i);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = substr4(&s, find_str_pos_i, find_str_len_i);
        elapsed_time = start_time.elapsed();
        print!("{:?}", elapsed_time.as_nanos());

        println!("");
    }

    // -------------------------------------------------------------------------

    println!("");
    println!("str_remove,str_remove2,str_remove3,str_remove4,str_remove5");

    for _ in 0..loops {
        start_time = Instant::now();
        let _result: String = str_remove(&s, find_str_pos_u, find_str_len_u);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = str_remove2(&s, find_str_pos_u, find_str_len_u);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = str_remove3(&s, find_str_pos_u, find_str_len_u);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = str_remove4(&s, find_str_pos_u, find_str_len_u);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = str_remove5(&s, find_str_pos_u, find_str_len_u);
        elapsed_time = start_time.elapsed();
        print!("{:?}", elapsed_time.as_nanos());

        println!("");
    }

    // -------------------------------------------------------------------------

    println!("");
    println!("str_concat,format,add");

    for _ in 0..loops {
        start_time = Instant::now();
        let _result: String = str_concat!(&s, &s, &s);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = format!("{}{}{}", s, s, s);
        elapsed_time = start_time.elapsed();
        print!("{:?},", elapsed_time.as_nanos());

        start_time = Instant::now();
        let _result: String = s.clone() + &s + &s; // first string is moved, from second borrowed
        elapsed_time = start_time.elapsed();
        print!("{:?}", elapsed_time.as_nanos());

        println!("");
    }

    // -------------------------------------------------------------------------
}
