#![allow(unused_variables, unused_mut)]

pub fn borrowing1() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // Impossible car il y a déjà une référence mutable vers s
    // println!("{} {}", r1, r2);
}

pub fn borrowing2() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 devient hors scope

    let r2 = &mut s; // Possible car il n'y a plus de référence vers s
    println!("{}", r2);
}

pub fn borrowing3() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // Possible car il y a uniquement des références immutables vers s
}

pub fn borrowing4() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // Impossible car il y a déjà des référence immmutable vers s
    // println!("{} {} {}", r1, r2, r3);
}

pub fn borrowing5() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s; // Possible car r1 et r2 ne seront plus utilisée parès le println! précédent
    println!("{}", r3);
}
