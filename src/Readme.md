# 🧙 Little-RPG - Jeu de rôle CLI en Rust

![Rust Version](https://img.shields.io/badge/Rust-1.88-orange?logo=rust&logoColor=white&style=for-the-badge)

> Un projet en ligne de commande écrit en Rust, inspiré des mécaniques de RPG classiques et modernes (Dark Souls, roguelike, RPG papier). Le jeu combine narration, exploration, combat stratégique, loot et évolution du personnage.

---

## 🎯 Objectifs du projet

- Développer un RPG textuel complet en Rust
- Apprendre les bonnes pratiques en structuration de projet Rust
- Explorer des concepts avancés : inventaire, IA ennemie, donjons, sauvegarde JSON...
- Créer un gameplay simple mais accessible

---

## 🔧 Fonctionnalités prévues

### ✅ Phase 1 – Socle de gameplay
- [x] Structuration des entités `Player` et `Enemy`
- [x] Système de combat de base
- [x] Expérience (XP) et montée de niveau
- [x] Inventaire du joueur (objets, potions)
- [x] Équipement (armes, armures, accessoires)
- [x] Types d’ennemis :
  - Lambda
  - Nommés
  - Élites
  - Légendaires
- [ ] Système de coups critiques / differentes stats HP, Crit, Vigueure (pour blocage par ex ou Endurance)
- [x] Cheat code `demonic_eye` (tue tous les ennemis dans la salle actuelle)


### ⚔️ Phase 2 – Progression & profondeur

- [ ] Loot d’objets et d’XP à la mort d’un ennemi
- [ ] Systeme de une et deux main pour armes et bouclier
- [ ] blocage et parade ex bouclier bloque un certain pourcentage, une longsword moins
- [ ] systeme de choix attaque vs defense ? type pierre feuille ciseaux ?
  - Idee d'un systeme ou l'ennemi peut lancer une attaque normale ou par exemple p60 vs e80 le player prends 20 degat
  - si player choisis de defendre il bloque une partie des 80 et riposte automatiquement ?
  - si player attaque un ennemie qui se defend alors c'est l'ennemie qui riposte
  - voir pour ia ennemy ?
  - pattern d attaque selon les enemies ? avec les lambda identique et par exemple les autres varient plus ?
- [ ] Ressources secondaires : vitalité, blocage, attaque


### 🧭 Phase 3 – Exploration et navigation
- [ ] Système de zones
- [ ] Systeme de Capacité d'Ennemi
- [ ] Menus de navigation (explorer, stats, quitter…)
- [ ] Syteme de Donjons
- [ ] Salles de combat ou d’événement
- [ ] Points de repos (type feux de camp Dark Souls)

### 📜 Phase 4 – Immersion & narration
- [ ] Texte d’introduction et narration dynamique
- [ ] Succès / exploits (ex. battre un légendaire, finir sans soins…)

### 🏆 Phase 5 – Fin & post-game
- [ ] Sauvegarde et chargement de partie (via `serde_json`)
- [ ] Fin du jeu (victoire ou boss final)
- [ ] Déblocage du **mode Hardcore**
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