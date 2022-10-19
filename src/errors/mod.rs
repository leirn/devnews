#![allow(dead_code)]

pub enum Nombres {
    Un,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
}

pub fn traitement_des_cas(nombre: Nombres) {
    match nombre {
        Nombres::Un => println!("1"),
        Nombres::Deux | Nombres::Trois => println!("2 ou 3"),
        Nombres::Quatre => println!("4"),
        _ => println!("Plus que 4)"),
    }
}

pub fn traitement_des_erreurs() {
    // Traitement de base
    let i = "1".parse::<u8>();
    let _k = i.unwrap();
    // Le code panic dans ce cas en raison de l'overflow
    // thread 'main' panicked at 'called `Result::unwrap()`
    // on an `Err` value: ParseIntError { kind: PosOverflow }
    let j = "1000".parse::<u8>();
    let _k = j.unwrap();

    // Même comportement, mais avec un message spécifique pour
    // aider au débuggage
    let j = "1000".parse::<u8>();
    let _k = j.expect("Parsing de j ne fonctionne pas");

    // Pas de panic, le code continue son exécution,
    // mais k a une valeur par défaut
    let j = "1000".parse::<u8>();
    let _k = j.unwrap_or_default(); // Default = 0 pour un u8
    let j = "1000".parse::<u8>();
    let _k = j.unwrap_or(255);

    let j = "1000".parse::<u8>();
    match j {
        Ok(_k) => (), // Faire un traitement sur j
        Err(_e) => (), // Faire un traitement adapté
                       // pas nécessairement bloquant pour l'application
    }
}
