# Little-Adventure - Jeu d'Aventure CLI en Rust

![Rust Version](https://img.shields.io/badge/Rust-1.92-orange?logo=rust&logoColor=white&style=for-the-badge)

> Un projet en ligne de commande écrit en Rust, inspiré des mécaniques de RPG classiques et modernes (Dark Souls, roguelike, RPG papier). Le jeu combine narration, exploration, combat stratégique, loot et évolution du personnage.

---

## 🎯 Objectifs du projet

- Développer un RPG textuel complet en Rust
- Apprendre les bonnes pratiques en structuration de projet Rust
- Progresser en Game Dev
- Progresser en Rust Idiomatique
- Explorer l'ecosysteme de Rust : Serde, Ratatui,...
- Créer un gameplay simple mais accessible
- Mener un projet Game Dev itératif qui abouti à un jeu fini

---

## Histoire et Deroule du jeu

Le jeu se dans un univers mediaval fantastique, le joueur se reveille sans souvenir de son passé, il doit dans un premier temps trouver un village ou se reposer.
cela fait office de phase de tutoriel.   
Ensuite le coeur du jeu est d'alterner entre exploration, combat, ville et donjon afin de découvrir les mystères qui entourent le joeur et de découvrir les secrets de l'univers.

---

## Gameplay

Le jeu repose sur un système **tour par tour** avec progression du personnage, gestion de l’XP et possibilité de sauvegarde.

### Choix de l'action

Le joueur choisit sont Action parmis les suivantes :
 - **Explorer** : explore les environs pour trouver des objets ou des ennemis.
 - **Ville** : visitez les villes pour acheter des objets ou des équipements.
 - **Donjon** : pénétrez dans des donjons pour trouver des objets ou des ennemis.
 - **Voyage Rapide** : retourner dans une ville déja visitée.  
 
#### Exploration

le mode exploration permet au joueur de découvrir les alentours, de trouver des ennemis, des objets, des pnj, des actions de quetes.   
c'est en explorant que l'on decouvre des nouvelles villes et des donjons.

#### Ville

La ville est un lieu où le joueur peut acheter des objets ou des équipements et se reposer.   
Elle se compose de plusieurs éléments : 
- **Marché** : achetez/vendre des objets et équipements.
- **Auberge** : réparez votre équipement et récupérez des points de vie.
- **Alchimiste** : concotez des potions et elixirs(buffe et debufs temporaires).
- **Evennements Uniques** : Pnj de Quête et événements de l'histoire.

#### Donjon

Le donjon est un lieu où le joueur peut rencontrer des ennemis et trouver des objets.   
Il se compose de plusieurs éléments : 
- **Entrée** : commencez votre assaut.
- **Chambres** : explorez les chambres pour trouver des objets ou des ennemis.
- **Boss** : combattez le boss pour sortir du donjon.
- **Ennemis** : Les ennemis sont répartis en differents tiers. : 
  - **Tiers 1** : Les ennemis lambda de la faction.
  - **Tiers 2** : Les ennemis uniques Nommés de la faction.
  - **Tiers 3** : Les ennemis uniques Elite de la faction.
  - **Tiers 4** : Les ennemis uniques de la faction de rang Boss, il s'agit des Boss et Legendaires.

### Combat

### Inventaire

#### Armes
#### Armure
#### Potion et Elixir

### Joueur

#### Réputation

### Progression

#### Experience
#### Réputation
#### Quêtes

---

### Progression
- **Level Up** : améliore vos statistiques et débloque de nouvelles capacités.
- **XP** : obtenue en combattant des ennemis (même en cas de fuite, l’XP est conservée).
- **Sauvegarde** : permet de reprendre la partie à tout moment.

---

### Système de combat
Combat structuré en **choix tactiques** à chaque tour :

- **Fuite** : abandonne le loot en cours mais conserve l’XP gagnée.
- **Défense** :
  - Les ennemis vous attaquent.
  - Selon votre bouclier, vous pouvez bloquer et **riposter** (possible uniquement avec un bouclier équipé).
  - Action disponible uniquement pour les armes **à une main** attaquant une fois par tour.
- **Attaque** :
  - Vous attaquez un ennemi selon le **pattern** de votre arme.
- **Parade** :
  - Bloque partiellement et riposte selon votre arme.
  - Accessible aux armes **à deux mains** ou aux armes avec le trait *Parade* (ex. : hache).
- **Inventaire** :
  - Utilisez un objet (ex. : potion).

---

### IA des ennemis
- **Patterns d’attaque** spécifiques selon le type d’ennemi.
  - **Tier 1** → Mono pattern reptété. exemple ["attaque", "bloque"]
  - **Tier 2** → Mono pattern reptété mais unique à l'ennemi.
  - **Tier 3** → Multi Pattern Possède un/des pouvoirs spéciaux.
  - **Tier 4** → Multi pattern, Possède des pouvoirs spéciaux.


## Objectifs V1

Le but de la v1 est d'avoir un projet minimalite mais standalone avec un debut et une fin, pas forcément scénarisée.  
le joueur commence une partie il ne peux qu'explorer  
- [ ] Exploration : boucle d'exploration fonctionnelle, le joeur peut  
  - [ ] Trouver un objet
  - [ ] Combattre
  - [ ] Voyager 
- [ ] Village :
  - [ ] le joueur peut se soigner a l'auberge
  - [ ] l'aubergiste donne une quete au joueur (tuer 10 gobelins pour avoir la map du donjon)
  - [ ] le joeur peut intéragir entre le différents éléments de la ville (magasin, auberge, repartir en exploration)
- [ ] Quest
  - [ ] Sur les gobelin on peut drop la clé du donjon (elle est unique)
  - [ ] Reward peut etre objet, xp 
  - [ ] traqueur de progres 
- [ ] Combat
  - [ ] Combat fonctionnel tour par tour
  - [ ] Loot des ennemis avec table de loots
- [ ] Donjon
  - [ ] Donjon fonctionnel avec plusieurs chambre, gamestate donjon, possibilité de quitter le donjon
  - [ ] Boss
  - [ ] message de gg credit v1
  
  
    
    
  

---

## 🛠️ Technologies utilisées

- Langage : **Rust**
- Librairies :
  - [`rand`](https://crates.io/crates/rand) – génération aléatoire
  - [`serde`](https://crates.io/crates/serde), [`serde_json`](https://crates.io/crates/serde_json) – sauvegarde JSON
  - (à venir) [`colored`] – mise en forme terminal
  - (à venir) [`ratatui`] – UI terminal plus avancée

---

## 🚀 Dev Lancer le jeu

```bash
cargo run
```


## Auteur

Quentin Lachery.
