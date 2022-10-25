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

# Structs

Rust n’est pas un langage orienté objet.
On peut toutefois y créer des structures, et associer des fonctions à ces structures

Les structures de Rust ressemblent à celles du C. On peut ensuite  y adjoindre des fonctions, statiques ou non : voir l'exemple du Rectangle

# Traits

Rust n'implémente pas le pattern d'*héritage*, mais se base sur le pattern de *composition*. Plutôt que d'hériter un composant d'un parent, on lui définit des comportements, les __traits__.

Il est possible de définir des comportements communs à des structs différentes via les *traits* : voir l'exemple des articles

[Sommaire]


  [Sommaire]: ../../Readme.md