---
title: "Rust introduction"
author: "Laurent Vromman"
date: "10/25/2022"
output: html_document
---

<style type="text/css">
  body{
  font-size: 16pt;
}
</style>

# Les différentes fonctionnalités de Rust

- Orienté programmation fonctionnelle
- Orienté bas niveau : contrôle précis sur la machine, mais sans les erreurs de mémoire ou de concurrence
- Cargo : logiciel de gestion des packages plébiscité par les développeurs
- L'environnement de dev devient rapidement ton meilleur ami : Messages d'erreurs précis, suggestions de correction, linter (Clippy)
- Bien souvent en Rust, un code qui compile est un code qui fonctionne !

# Les différentes caractéristiques du code Rust

- [Mutability]
- Fortement typé, inférence des types, pas de casting implicite
- [Ownership] : gestion du borrowing et des durées de vie des variables
- [Borrowing] : gestion des références sur une même donnée
- Gestion systématique des [erreurs]
- Built-in test framework
- [Structs], [Traits]

Et bien d'autres
- Exécution synchrone ou asynchrone (await / future)
- Macros (appliquée sur l'AST, variadiques et pourquoi pasrécursives)
- Closures (équivalent de lambda functions), Iterator
- [Types génériques](https://doc.rust-lang.org/book/ch10-01-syntax.html)

# Vous souhaitez en apprendre plus ?
- [Learn Rust](https://www.rust-lang.org/learn)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rustacean Discord](https://discord.gg/rust-lang-community)
- [Rust embedded book](https://docs.rust-embedded.org/book/intro/index.html?ref=hackr.io)
- And many others !

  [Mutability]: src/mutability/mutability.md
  [Ownership]: src/ownership/ownership.md
  [Borrowing]: src/borrowing/borrowing.md
  [erreurs]: src/erorrs/errors.md
  [Structs]: src/traits/traits.md
  [Traits]: src/traits/traits.md
