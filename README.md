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
* new -> instancie une image avec : le format, la taille, la valeur max des pixels, les pixels
* save -> sauvegarde l'image dans un fichier dont l'adresse et le format sont passé en paramètre (exemple : "mon_chemin\\ma_nouvelle_image.ppm")
* invert -> lance la fonction "invert" des pixels pour tout les pixels de l'image
* grayscale -> lance la fonction "grayscale" des pixels pour tout les pixels de l'image
* eq -> compare les pixels de chaques images
* format, width, height, max, pixels -> getters

### Main
Dans la fonction main est exécuté une commande simple de création d'une structure image puis sauvegarde de celle-ci dans un nouveau fichier.
Ne pas oublier de changer les chemins absolus!!

## Lancement des tests
Pour pouvoir build le projet les lignes "//extern crate lib_image;" dans le fichier "lib_image.rs" (ligne 3) et "//extern crate lib_pixel;" dans le fichier "lib_pixel.rs" (ligne 3) doivent être décommentées. Puis on peu faire "cargo run" et "cargo build".
Pour pouvoir lancer les tests (dans les fichiers lib_pixels.rs et lib_image.rs) il faut recommenter ces  lignes et lancer la commande "cargo test".

## Documentation utile
* https://doc.rust-lang.org
* https://blog.guillaume-gomez.fr/Rust