# Première séance :
# TP3 RUST : Découverte de l'embarqué
## Partie I : BSP pour nucléo L476RG

Nous avons créé un fichier bsp.rs pour représenter de manière générique les différents modules de notre carte.\
Bargraph LED, encodeur etc...

## Partie II : Drivers
### Bargraph
Nous avons passé un temps considérable à écrire le driver pour le bragraph de LED (à cause notamment d'un problème de type).
Néanmoins, nous avons réussi à la fin de la séance à allumer les leds, et même à faire un chenillard.\
<img width="525" height="1000" alt="image" src="https://github.com/user-attachments/assets/583d521b-d6ae-4f76-9e15-8398f8855d8d" />

Le temps est écoulé, nous n'avons pas pu aller plus loin.

# Deuxième séance :
### Gamepad
Nous affichons correctement le polling des boutons, nous avons également utilisé la fonction "is_pressed" pour activer et desactiver le bargraph avec le bouton central.\
<img width="358" height="88" alt="image" src="https://github.com/user-attachments/assets/2f5457f2-e3d8-43e0-8af9-5053320af7b0" />

### Encodeur
Nous réussisons à faire fonctionner l'encodeur et à récuperer ses valeures. Pour le tester, nous l'utilisons pour controler le bargraph. tourner l'encodeur vers la droite allume de plus en plus de leds, et vers la gauche les éteint.
