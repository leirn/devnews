---
title: "Rust mutability"
author: "Laurent Vromman"
date: "10/25/2022"
output: html_document
---

<style type="text/css">
  body{
  font-size: 16pt;
}
</style>

# Ownership

## Ownership Rules
- Chaque valeur (donnée en mémoire) a un **propriétaire**
- Il ne peut y avoir qu'un seul propriétaire à un instant donné
- Quand le propriétaire est hors scope, la valeur est droppée

## Example
Ce code produit une erreur car la donnée a été transférée à s2 :
```rs
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

Ce code fonctionne car la donnée est dupliquée en mémoire, s1 et s2 sont chacun propriétaires de leurs propres données
```rs
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}, world!", s1);
```

Semble pénible au départ, mais permet d’avoir un programme memory-safe sans allouer soit même la mémoire, sans garbage collector, avec la performance du C

C’est règle sont vérifiées à la compilation et non à l’exécution

Nécessite de bien comprendre les allocations mémoires



[Sommaire]


  [Sommaire]: ../Intro.md