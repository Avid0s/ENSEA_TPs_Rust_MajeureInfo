# TP2 RUST
Ce projet a pour but de prendre en main le langage RUST.\
Nous essayons de "construire" un compilateur Logo, un langage très simple.

## Partie I : 
Nous essayons de fabriquer la grammaire du langage.\
En voici un exemple :

<program> ::= <command> <program> | ""

<command> ::= <block>
            | <order>
            | <state>
            | <loop>

<block> ::= "[" <command-list> "]"

<order> ::= "forward" <number>
         | "backward" <number>
         | "left" <number>
         | "right" <number>

<state> ::= "penup"
        | "pendown"

<loop> ::= "repeat" <number> <block>

<number> ::= <digit> <number>
           | <digit>

<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"


Nous pouvons ainsi décrire les actions nécessaires pour dessiner un carré.\
cf suqare.txt\

Pour simplifier, nous appliquerons cette grammaire : 
<program> ::= <command> <program> | ""
<command> ::= <order> <number>
<order> ::= "forward" | "backward" | "left" | "right"
<number> ::= [0-9]+

## Partie II : Analyse lexicale et syntaxique :

### a) Nous utilisons la bibliothèque Santiago pour construire notre analyseur logo (lexer), qui va reconnaître les différents blocs définis dans la grammaire : 

Cela fonctionne :\
<img width="250" height="709" alt="image" src="https://github.com/user-attachments/assets/6536466a-a913-44c9-ac87-ea7a249e7384" />\

Nous avons bien rajouté la consigne pour ignorer les espaces.

### b) Nous écrivons maintenant la grammaire (parser), fonction qui se charge de lier les différentes briques de base:
<img width="356" height="177" alt="image" src="https://github.com/user-attachments/assets/471feb5b-0753-4ad3-8ad8-b1bf2cc20ce1" />\


### c) En testant la fonction pour fabriquer un arbre de syntaxe abstraite, nous sommes parvenus de manière surprenante à faire un stack overflow :
<img width="205" height="24" alt="image" src="https://github.com/user-attachments/assets/6e94a049-b5b2-482f-b1bd-f574259b0421" />\
ça marche plus

Heureusement, nous avons pu corriger le problème\
<img width="223" height="111" alt="image" src="https://github.com/user-attachments/assets/9570cd0c-c612-4a1a-bbac-0b94cfa2832d" />

## Partie III : Compilateur Logo vers SVG :

Nous n'avons pas pris de captures d'écran, mais nous avons réussi à écrire une fonction permettant de créer un fichier SVG en fonction des instructions transmises selon la grammaire définie plus haut. grâce à la bibliothèque svg_fmt et en utilisant la commande File::open pour créer le fichier, nous sommes parvenus à créer un fichier SVG pour desssiner un carré.

> Nous arrivons à court de temps, donc nous terminons là-dessus.

