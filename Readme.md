# Little-Adventure - Jeu d'Aventure CLI en Rust

![Rust Version](https://img.shields.io/badge/Rust-1.88-orange?logo=rust&logoColor=white&style=for-the-badge)

> Un projet en ligne de commande écrit en Rust, inspiré des mécaniques de RPG classiques et modernes (Dark Souls, roguelike, RPG papier). Le jeu combine narration, exploration, combat stratégique, loot et évolution du personnage.

---

## 🎯 Objectifs du projet

- Développer un RPG textuel complet en Rust
- Apprendre les bonnes pratiques en structuration de projet Rust
- Explorer des concepts avancés : inventaire, IA ennemie, donjons, sauvegarde JSON...
- Créer un gameplay simple mais accessible

---

## Histoire et Deroule du jeu

Le jeu se déroule à travers **5 Donjons** et se compose de deux actes principaux, suivis d’un épilogue et des crédits.

---

### Acte 1
- **Donjons** : 5 salles chacun.
- **Progression** :
  - **Salle 1** : 1 ennemi.
  - **Salles 2 à 4** : 1 à 3 ennemis (possibilité d’apparition d’**ennemis nommés** ou **Élite**).
  - **Salle 5** : 1 à 3 ennemis + **Boss** (possibilité d’apparition **Légendaire**).
- **Types d’ennemis** :
  - Gobelins  
  - Squelettes  
  - Humains  

---

### Acte 2
- **Donjons** : 10 salles chacun.
- **Progression** :
  - **Salles 1 à 5** : 3 à 5 ennemis.
  - **Salles 6 à 9** : 5 à 10 ennemis.
  - **Salle 10** : 2 ennemis **Élite** + **Boss** (possibilité d’apparition **Légendaire**).
- **Types d’ennemis** :
  - Vampires  
  - Démons  

---

### Fin
- **Épilogue** : conclusion de l’histoire.
- **Crédits** : remerciements mon nom mdr.


## Gameplay

Le jeu repose sur un système **tour par tour** avec progression du personnage, gestion de l’XP et possibilité de sauvegarde.

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
  - Exemple : **Gobelin Lambda** → attaque uniquement.
  - **Boss** → possèdent des pouvoirs spéciaux.


## 🔧 Fonctionnalités prévues

### ✅ Phase 1 – Socle de gameplay
- [x] Structuration des entités `Player` et `Enemy`
- [ ] Système de combat **tour par tour** avec choix tactiques :
  - [ ] Fuite (conserve l’XP mais perd le loot)
  - [ ] Défense (bouclier requis, riposte si arme à une main)
  - [ ]Attaque (pattern selon l’arme)
  - [ ] Parade (riposte spécifique, armes à deux mains ou avec trait *Parade*)
  - [ ] Utilisation d’objets via l’inventaire
- [x] Expérience (XP) et montée de niveau
- [x] Inventaire du joueur (objets, potions)
- [x] Équipement (armes, armures, accessoires)
- [x] Types d’ennemis :
  - Lambda (attaquent simplement)
  - Nommés
  - Élites
  - Légendaires (possibles en salle finale)
- [ ] Système de coups critiques
- [ ] Différentes statistiques de personnage (HP, Critique, Vigueur, Endurance/blocage)
- [x] Cheat code `demonic_eye` (tue tous les ennemis dans la salle actuelle)

---

### ⚔️ Phase 2 – Progression & profondeur
- [ ] Loot d’objets et d’XP à la mort d’un ennemi
- [ ] Gestion des armes à une main / deux mains et compatibilité avec bouclier
- [ ] Blocage et parade avec valeurs spécifiques (ex. bouclier bloque plus qu’une épée longue)
- [ ] Système d’IA ennemie basé sur des **patterns** :
  - Lambda → patterns simples
  - Autres types → patterns variés et adaptatifs
- [ ] Ressources secondaires : vitalité, blocage, attaque
- [ ] Pouvoirs spéciaux pour certains ennemis (notamment Boss)

---

### 🧭 Phase 3 – Exploration et navigation
- [ ] Système de zones et progression par **biomes**
- [ ] Système de donjons avec structure par salles :
  - Acte 1 → 5 salles
  - Acte 2 → 10 salles
  - Types et nombres d’ennemis selon la salle
- [ ] Menus de navigation (explorer, voir stats, quitter…)
- [ ] Salles de combat ou d’événement
- [ ] Points de repos (type feux de camp Dark Souls)

---

### 📜 Phase 4 – Immersion & narration
- [ ] Texte d’introduction et narration dynamique
- [ ] Succès / exploits (ex. battre un légendaire, finir un acte sans soins)
- [ ] Épilogue et crédits

---

### 🏆 Phase 5 – Fin & post-game
- [ ] Sauvegarde et chargement de partie (via `serde_json`)
- [ ] Fin du jeu (victoire ou boss final)
- [ ] Déblocage du **mode Hardcore** :
  - Suppression de la sauvegarde en cas de mort
  - Fin alternative

---

## 🛠️ Technologies utilisées

- Langage : **Rust**
- Librairies :
  - [`rand`](https://crates.io/crates/rand) – génération aléatoire
  - [`serde`](https://crates.io/crates/serde), [`serde_json`](https://crates.io/crates/serde_json) – sauvegarde JSON
  - (à venir) [`colored`] – mise en forme terminal

---

## 🚀 Lancer le jeu

```bash
cargo run
```

Le jeu est actuellement en cours de développement. Seules certaines fonctionnalités de base sont disponibles.


## 👨‍💻 Auteur

Projet réalisé par Quentin Lachery dans le cadre d’un apprentissage Rust appliqué à la conception de jeux vidéo en ligne de commande.