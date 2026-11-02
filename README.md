# développé par Margotte Storme et Ylian Bsila
# projet_rust_3SI4

Ce projet est un gestionnaire de mots de passe simple écrit en **Rust**.  
Il permet d'ajouter et de récupérer des mots de passe stockés de manière sécurisée.

## 🚀 Fonctionnalités
- 🔒 Chiffrement des mots de passe avec **AES-256-GCM** (via la bibliothèque \`ring\`).
- 🧂 Utilisation de sel et nonce aléatoires
- 📂 Stockage des mots de passe sous forme de fichier JSON chiffré.
- 🔑 Protection par mot de passe maître : hash salé (PBKDF2-HMAC-SHA256) stocké localement, jamais le mot de passe en clair.
- 🖥️ Interface graphique développée avec `eframe` / `egui`
- 🧼 Code formaté avec `cargo fmt`

## 📦 Installation

Assurez-vous d'avoir **Rust** installé. Si ce n'est pas le cas, installez-le avec :

\`\`\`sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
\`\`\`

Puis, clonez ce projet et installez les dépendances :

\`\`\`sh
git clone https://github.com/margottestorme/projet_rust_3SI.git
cd projet_rust_3SI/gestionnaire_mdp
cargo build --release
\`\`\`

## 🛠️ Utilisation

Lancez l'application avec :

\`\`\`sh
cargo run
\`\`\`

Au premier lancement, l'application demande de créer un **mot de passe maître** (avec confirmation). Ce mot de passe n'est jamais stocké en clair : seul un sel et un hash PBKDF2-HMAC-SHA256 sont enregistrés dans `master.key`, et c'est ce mot de passe qui sert à dériver la clé de chiffrement AES-256-GCM de chaque entrée.

Aux lancements suivants, il faut ressaisir ce même mot de passe maître pour déverrouiller l'application et pouvoir déchiffrer les entrées existantes.

Vous pourrez alors :
1. **Ajouter un mot de passe** pour un site.
2. **Récupérer un mot de passe** en entrant le nom du site.
3. **Quitter** le programme.

## 📂 Fichiers générés

- \`master.key\` → Sel + hash du mot de passe maître (jamais le mot de passe en clair).
- \`passwords.json.enc\` → Contient les mots de passe chiffrés.
- \`target/release/password-manager\` → Binaire compilé (si buildé en release).

## 🛡️ Sécurité

- **Les mots de passe sont stockés chiffrés avec AES-256-GCM**, avec une clé dérivée du mot de passe maître via PBKDF2-HMAC-SHA256 (100 000 itérations) et un sel aléatoire par entrée.
- Le mot de passe maître n'est jamais écrit sur disque : seul un hash salé est conservé pour vérifier la saisie lors de la connexion.
