# développé par Margotte Storme et Ylian Bsila
# projet_rust_3SI4

Ce projet est un gestionnaire de mots de passe simple écrit en **Rust**.  
Il permet d'ajouter et de récupérer des mots de passe stockés de manière sécurisée.

## 🚀 Fonctionnalités
- 🔒 Chiffrement des mots de passe avec **AES-256-GCM** (via la bibliothèque \`ring\`).
- 🧂 Utilisation de sel et nonce aléatoires
- 📂 Stockage des mots de passe sous forme de fichier JSON chiffré.
- 🔑 Protection par mot de passe maître (implémentation future).
- 🖥️ Interface graphique développée avec `eframe` / `egui`
- 🧼 Code formaté avec `cargo fmt`

## 📦 Installation

Assurez-vous d'avoir **Rust** installé. Si ce n'est pas le cas, installez-le avec :

\`\`\`sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
\`\`\`

Puis, clonez ce projet et installez les dépendances :

\`\`\`sh
git clone https://github.com/ton_github/password-manager-rust.git
cd password-manager-rust
cargo build --release
\`\`\`

## 🛠️ Utilisation

Lancez l'application avec :

\`\`\`sh
cargo run
\`\`\`

Vous pourrez alors :
1. **Ajouter un mot de passe** pour un site.
2. **Récupérer un mot de passe** en entrant le nom du site.
3. **Quitter** le programme.

## 📂 Fichiers générés

- \`passwords.json.enc\` → Contient les mots de passe chiffrés.
- \`target/release/password-manager\` → Binaire compilé (si buildé en release).

## 🛡️ Sécurité

- **Les mots de passe sont stockés chiffrés avec AES-256-GCM**.
