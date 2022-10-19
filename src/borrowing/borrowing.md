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

# Borrowing

## Borrowing Rules and references
- A tout instant on peut avoir, pour une même donnée :
  - Une référence mutable
  - Ou bien plusieurs références immutables
- Une référence doit toujours être valide, c'est-à-dire pointer vers une donnée existante

Il est impossible d'avoir simultanément une référence mutable et une référence immutable vers une même donnée.

Ces principes permettent de garantir la non concurrence dans l'accès aux données.

[Sommaire]


  [Sommaire]: ../../Readme.md