fn median(input: &mut [i32]) -> i32{
    println!("Hello");
    println!("{:?}", input);
    input.sort();
    println!("{:?}", input);
    println!("{}", input.len());
    let n = input.len();
    if n % 2 == 0 {
        return (input[n/2 -1 ]+input[n/2])/2;
    } else {
        return input[n/2];
    }
}
use std::collections::HashMap;

fn mode(input: &mut [i32]) -> i32 {
    let mut map = HashMap::new();

    for elem in input {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }

    let mut highest: i32 = 0;
    for (_, value) in map.iter() {
        if *value > highest {
            highest = *value;
        }
    }

    return highest;
}

fn pig_latin(s: &str) {
    let last_char = s.chars().last().unwrap();

    let isVowel = match last_char {
        'o' => true,
        'e' => true,
        'i' => true,
        'a' => true,
        'u' => true,
        other => false, 
    };

    if isVowel {
        println!("found vowel");
        let c = s.chars().next().unwrap();
        let s1 = &s[1..];
        let mut st = String::from(s1);
        st.push_str("-");
        st.push(c);
        st.push_str("ay");
        println!("{st}");
    } else {
        let mut st = String::from(s);
        st.push_str("-hay");
        println!("{st}"); 
    }
}

fn main(){
    let mut input: [i32; 6] = [8,2,1,3,2,5];
    let med = median(&mut input);
    println!("{med}");
    let mo = mode(&mut input);
    println!("{mo}");

    let s1 = "Hello";
    let s2 = "what";
    pig_latin(s1);
    pig_latin(s2);
}