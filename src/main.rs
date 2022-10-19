mod borrowing;
mod errors;
mod mutability;
mod ownership;
mod traits;

fn main() {
    println!("Hello, world!");
    mutability::mutability1();
    mutability::mutability2();
    mutability::mutability3();

    ownership::ownership1();
    ownership::ownership2();

    borrowing::borrowing1();
    borrowing::borrowing2();
    borrowing::borrowing3();
    borrowing::borrowing4();
    borrowing::borrowing5();

    traits::structs::play_with_rectangle();
    traits::traits::traits();

    errors::traitement_des_cas(errors::Nombres::UN);
    errors::traitement_des_erreurs();
}
