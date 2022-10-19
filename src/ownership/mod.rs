#![allow(dead_code, unused_variables)]

pub fn ownership1() {
    // La structure String est allouée en heap
    let s1 = String::from("hello");
    // Impossible d'avoir deux propriétaires pour une
    // même donnée, la propriété est transférée à S2
    let s2 = s1;
    // println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn ownership2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
