pub fn mutability1() {
    let x = 5_u8; // x est immutable
    println!("The value of x is: {x}");
    // x = 6; // Refusera de compiler
    println!("The value of x is: {x}");
}

pub fn mutability2() {
    let mut x = 5; // x est immutable
    println!("The value of x is: {x}");
    x = 6_u16; // Compilation fonctionnelle
    println!("The value of x is: {x}");
}

pub fn mutability3() {
    let x = 5; // x est immutable
    println!("The value of x is: {x}");
    let x = 6; // shadowing : fonctionne car x est redéfini
    println!("The value of x is: {x}");
}

pub fn mutability4() {
    // Le type de const doit être explicite et non inféré
    const x: u32 = 5;
    println!("The value of x is: {x}");
    // x = 6; // on ne peut pas modifier une constante
    println!("The value of x is: {x}");
}
