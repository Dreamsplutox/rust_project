# GROUPE 6 : 
- NEZONDET-RENAUD Nathanael
- SIMON Arnaud
- FOMBA Soumaila

## Sujet
https://rust-esgi.github.io/IABD1/big_project.html

## Brève explication de code
### lib_pixel
Module pixel avec derive(Debug, Clone, Copy) -> Module à importer dans main pour gérer les pixels
Méthodes :
* new -> instancie un pixel avec 3 valeurs entre 0 et 255 qui représentent RGB
* red, green, blue -> getters
* display -> retourner le pixel dans une string étoffée (parenthèses ajoutées)
* invert -> fait une inversion bit à bit avec "!"
* grayscale -> niveau de gris, moyenne des valeurs RGB sur red, green et blue
* display_to_byte -> renvoie un pixel au format String pour ensuite l'écrire dans u fichier ppm
* eq -> utilisation de PartialEq, vérifie qu'un pixel est égal à un autre en comparant leurs RGB

### lib_image
Module image avec derive(Debug)
Méthodes : 
* new ->
### main.rs

## Documentation utile
* https://doc.rust-lang.org
* https://blog.guillaume-gomez.fr/Rust