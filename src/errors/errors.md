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

# La gestion des cas et des erreurs en Rust

Le traitement des cas et des erreurs en Rust doit être:
- Explicite
- Exhaustif

Cela garantie la stabilité de l'application et prévient tout comportement imprévisible à l'exécution.

En conséquence, une application Rust aura beaucoup moins de problème de jeunesse lors des périodes de garantie, les cas anormaux ayant été pensés au départ, voire prévenus par le langage.


[Sommaire]


  [Sommaire]: ../../Readme.md