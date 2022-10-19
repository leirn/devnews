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

# Mutabilité

Les variables sont immutables (en lecture seule) par défaut

# Examples
```rs
let x = 5; // x est immutable

x = 6; // Won't compile, x is immutable by default

let mut x = 5; // x est mutable

x = 6; // Will now work
```

La non mutabilité par défaut est une vraie particularité de Rust
Seule, elle ne semble pas si intéressante
Pourtant, elle permet :
- De limiter les erreurs en ciblant dans une fonction les variables qui peuvent où non être modificables à un instant donné
- Couplée à la gestion du borrowing, c'est un des pilliers de la gestion mémoire de Rust


[Sommaire]


  [Sommaire]: ../Intro.md