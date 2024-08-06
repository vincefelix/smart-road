# smart-Road

"smart-Road" est une simulation de trafic routier qui modélise le comportement des véhicules et leur interaction aux intersections. Ce projet utilise la bibliothèque Macroquad pour le rendu graphique et la gestion des entrées utilisateur.
## Structure du Projet

Le projet est organisé de la manière suivante :
```bash
smart-Road/

├── assets/
│   ├── back_stat.png         # Image pour l'affichage des statistiques
│   ├── car_straight.png      # Texture pour les voitures allant tout droit
│   ├── car_right.png         # Texture pour les voitures tournant à droite
│   ├── car_left.png          # Texture pour les voitures tournant à gauche
│   └── ...                   # Autres assets
├── src/
│   ├── main.rs               # Point d'entrée principal du programme
│   ├── constant.rs           # Définition des constantes
│   ├── app/
│   │   ├── control.rs        # Gestion des entrées utilisateur
│   │   ├── init.rs           # Initialisation de l'application
│   │   ├── mod.rs            # Module d'application
│   │   └── statistics.rs     # Gestion des statistiques
│   ├── draw/
│   │   ├── background.rs     # Rendu du fond
│   │   ├── background_statistics.rs # Rendu du fond des statistiques
│   │   ├── car.rs            # Rendu des voitures
│   │   ├── mod.rs            # Module de dessin
│   │   └── path.rs           # Rendu des chemins des voitures
│   └── traffic/
│       ├── car.rs            # Logique de gestion des voitures
│       ├── curve.rs          # Gestion des courbes
│       ├── line.rs           # Gestion des lignes droites
│       ├── mod.rs            # Module de trafic
│       ├── path.rs           # Définition des chemins
│       └── path_collisions.rs # Détection des collisions sur les chemins
│       └── state.rs          # Gestion de l'état du trafic
├── Cargo.toml                # Fichier de configuration des dépendances Rust
└── README.md                 # Description du projet
```
## Logique Globale du Projet

Le projet smart-Road est conçu pour simuler un environnement de trafic urbain. Les voitures sont générées aléatoirement et suivent des chemins prédéfinis, incluant des lignes droites et des courbes. Les voitures sont représentées par des textures graphiques et se déplacent en fonction de leur direction et de leur vitesse.
### Fonctionnalités

1. Simulation de Trafic :
    - Génération de voitures aléatoires suivant différents chemins.
    - Affichage des voitures avec des textures adaptées à leur direction.

2. Gestion des Collisions :
    - Détection des collisions aux intersections.
    - Gestion des distances de sécurité entre les véhicules.

3. Statistiques Dynamiques :
    - Suivi du nombre de véhicules ayant traversé l'intersection.
    - Calcul des vitesses maximales et minimales des véhicules.
    - Détection des quasi-collisions.

4. Interface Graphique :
    - Affichage des statistiques sur un écran dédié.
    - Interface utilisateur pour interagir avec la simulation.

## Gestion des Collisions

La gestion des collisions dans smart-Road est centrée sur la détection et la prévention des accidents aux intersections. Le module path_collisions.rs contient la logique de détection des collisions, qui surveille les positions des voitures et vérifie les distances de sécurité requises. Lorsqu'une collision potentielle est détectée, les voitures s'arrêtent ou ralentissent pour éviter l'accident.

## Dépendances

Le projet "smart-Road" utilise les dépendances suivantes :

- Macroquad : Une bibliothèque pour le développement de jeux en 2D en Rust, version 0.4.4.
- rand : Générateur de nombres aléatoires pour diverses fonctionnalités, version 0.8.5.
- once_cell : Fournit un moyen de créer des cellules de données paresseuses et statiques, version 1.18.0.
- chrono : Bibliothèque de manipulation de date et heure, version 0.4.31.

## Installation

1. Prérequis : Assurez-vous d'avoir Rust installé sur votre machine.
2. Clonage du projet : Clonez ce dépôt sur votre machine locale.

```bash
git clone https://learn.zone01dakar.sn/git/vindour/smart-road
cd smart-Road
```
3. Installation des dépendances : Utilisez cargo pour installer les dépendances.

```bash
cargo build
```
4. Exécution de la simulation : Lancer la simulation avec la commande suivante :

```bash
cargo run
```
## Contribution

Les contributions sont les bienvenues ! Si vous avez des idées ou des améliorations, n'hésitez pas à ouvrir une issue ou une pull request.