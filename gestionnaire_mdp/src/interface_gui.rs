use crate::chiffrement::{
    chiffrer_donnee, dechiffrer_donnee, derive_key, generate_salt, verify_master_password,
};
use base64::Engine;
use base64::engine::general_purpose;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Read;

const FICHIER_JSON: &str = "./passwords.json.enc";
const FICHIER_MASTER: &str = "./master.key";

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    utilisateur: String,
    username: String,
    mdp: String,
    lien: String,
    nonce: String,
    salt: String,
}

#[derive(Serialize, Deserialize)]
struct MasterRecord {
    salt: String,
    hash: String,
}

pub struct MonApp {
    logged_in: bool,
    message: String,
    username: String,
    password: String,
    confirm_password: String,
    master_password: String,
    site: String,
    lien: String,
    mot_de_passe: String,
    afficher_ajout: bool,
    afficher_mdp: HashMap<usize, bool>, // Stocke l'état d'affichage des mots de passe
}

impl Default for MonApp {
    fn default() -> Self {
        MonApp {
            logged_in: false,
            message: String::new(),
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            master_password: String::new(),
            site: String::new(),
            lien: String::new(),
            mot_de_passe: String::new(),
            afficher_ajout: false,
            afficher_mdp: HashMap::new(),
        }
    }
}

impl eframe::App for MonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.logged_in {
                if self.afficher_ajout {
                    // Page pour ajouter un mot de passe
                    ui.heading("Ajouter un nouveau mot de passe");
                    ui.horizontal(|ui| {
                        ui.label("Site :");
                        ui.text_edit_singleline(&mut self.site);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Nom d'utilisateur :");
                        ui.text_edit_singleline(&mut self.username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Lien (optionnel) :");
                        ui.text_edit_singleline(&mut self.lien);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Mot de passe :");
                        ui.text_edit_singleline(&mut self.mot_de_passe);
                    });
                    if ui.button("Enregistrer").clicked() {
                        if self.site.is_empty() || self.mot_de_passe.is_empty() || self.username.is_empty() {
                            ui.label("Erreur : Le site, le nom d'utilisateur et le mot de passe sont obligatoires.");
                        } else {
                            if enregistrer_mdp(
                                &self.site,
                                &self.username,
                                &self.lien,
                                &self.mot_de_passe,
                                &self.master_password,
                            ) {
                                ui.label("Mot de passe enregistré avec succès !");
                                self.site.clear();
                                self.username.clear();
                                self.lien.clear();
                                self.mot_de_passe.clear();
                                self.afficher_ajout = false;
                            } else {
                                ui.label("Erreur lors de l'enregistrement du mot de passe.");
                            }
                        }
                    }
                    if ui.button("Retour").clicked() {
                        self.afficher_ajout = false;
                    }
                } else {
                    // Panneau principal
                    ui.heading("Panneau principal");
                    if ui.button("Ajouter un mot de passe").clicked() {
                        self.afficher_ajout = true;
                    }
                    match afficher_mdp() {
                        Ok(entries) => {
                            for (index, entry) in entries.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Site : {}", entry.utilisateur));
                                    ui.label(format!("Nom d'utilisateur : {}", entry.username));
                                    ui.label(format!("Lien : {}", entry.lien));
                                    if ui.button("Afficher/masquer mot de passe").clicked() {
                                        let afficher = self.afficher_mdp.entry(index).or_insert(false);
                                        *afficher = !*afficher;
                                    }
                                    if let Some(true) = self.afficher_mdp.get(&index) {
                                        let salt = general_purpose::STANDARD
                                            .decode(&entry.salt)
                                            .expect("Erreur de décodage du salt");
                                        let cle = derive_key(&self.master_password, &salt);
                                        let mdp_dechiffre = dechiffrer_donnee(
                                            &cle,
                                            &entry.mdp,
                                            &entry.nonce,
                                        )
                                        .unwrap_or_else(|| "Erreur déchiffrement".to_string());
                                        ui.label(format!("Mot de passe : {}", mdp_dechiffre));
                                    }
                                    if ui.button("Supprimer").clicked() {
                                        if supprimer_mdp(index) {
                                            ui.label("Mot de passe supprimé avec succès !");
                                        } else {
                                            ui.label("Erreur lors de la suppression.");
                                        }
                                    }
                                });
                            }
                        }
                        Err(err) => {
                            ui.label(format!("Erreur : {}", err));
                        }
                    }
                }
            } else if !std::path::Path::new(FICHIER_MASTER).exists() {
                // Première utilisation : création du mot de passe maître
                ui.heading("Première utilisation — créer un mot de passe maître");
                ui.horizontal(|ui| {
                    ui.label("Mot de passe maître :");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });
                ui.horizontal(|ui| {
                    ui.label("Confirmer :");
                    ui.add(egui::TextEdit::singleline(&mut self.confirm_password).password(true));
                });
                if ui.button("Créer").clicked() {
                    if self.password.is_empty() {
                        self.message = "Le mot de passe ne peut pas être vide.".to_string();
                    } else if self.password != self.confirm_password {
                        self.message = "Les mots de passe ne correspondent pas.".to_string();
                    } else if enregistrer_master_password(&self.password) {
                        self.master_password = self.password.clone();
                        self.logged_in = true;
                        self.message.clear();
                        self.password.clear();
                        self.confirm_password.clear();
                    } else {
                        self.message =
                            "Erreur lors de la création du mot de passe maître.".to_string();
                    }
                }
                if !self.message.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.message);
                }
            } else {
                // Page de connexion
                ui.heading("Connexion");
                ui.horizontal(|ui| {
                    ui.label("Mot de passe maître :");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });
                if ui.button("Se connecter").clicked() {
                    if verifier_master_password(&self.password) {
                        self.master_password = self.password.clone();
                        self.logged_in = true;
                        self.message.clear();
                        self.password.clear();
                    } else {
                        self.message = "Mot de passe maître incorrect.".to_string();
                    }
                }
                if !self.message.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.message);
                }
            }
        });
    }
}

// Fonction pour afficher les mots de passe
fn afficher_mdp() -> Result<Vec<Entry>, String> {
    let mut fichier = File::open(FICHIER_JSON).map_err(|_| "Fichier introuvable.")?;
    let mut contenu = String::new();
    fichier
        .read_to_string(&mut contenu)
        .map_err(|_| "Erreur de lecture du fichier.")?;

    let entrees: Vec<Entry> =
        serde_json::from_str(&contenu).map_err(|_| "Format JSON invalide.")?;

    Ok(entrees)
}

// Crée le mot de passe maître (premier lancement) : stocke un sel + un hash PBKDF2,
// jamais le mot de passe en clair.
fn enregistrer_master_password(password: &str) -> bool {
    let salt = generate_salt();
    let hash = derive_key(password, &salt);
    let record = MasterRecord {
        salt: general_purpose::STANDARD.encode(salt),
        hash: general_purpose::STANDARD.encode(hash),
    };

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FICHIER_MASTER)
    {
        Ok(mut fichier) => serde_json::to_writer_pretty(&mut fichier, &record).is_ok(),
        Err(_) => false,
    }
}

// Vérifie le mot de passe maître saisi contre le sel + hash stockés.
fn verifier_master_password(password: &str) -> bool {
    let mut fichier = match File::open(FICHIER_MASTER) {
        Ok(fichier) => fichier,
        Err(_) => return false,
    };

    let mut contenu = String::new();
    if fichier.read_to_string(&mut contenu).is_err() {
        return false;
    }

    let record: MasterRecord = match serde_json::from_str(&contenu) {
        Ok(record) => record,
        Err(_) => return false,
    };

    let salt = match general_purpose::STANDARD.decode(&record.salt) {
        Ok(salt) => salt,
        Err(_) => return false,
    };
    let hash = match general_purpose::STANDARD.decode(&record.hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    verify_master_password(password, &salt, &hash)
}

// Fonction pour enregistrer un mot de passe
fn enregistrer_mdp(
    site: &str,
    username: &str,
    lien: &str,
    mot_de_passe: &str,
    master_password: &str,
) -> bool {
    let salt = generate_salt();
    let cle = derive_key(master_password, &salt);
    let salt_str = general_purpose::STANDARD.encode(salt);
    let (mdp_chiffre, nonce_str) = chiffrer_donnee(&cle, mot_de_passe);

    let nouvelle_entree = Entry {
        utilisateur: site.to_string(),
        username: username.to_string(),
        mdp: mdp_chiffre,
        lien: lien.to_string(),
        nonce: nonce_str,
        salt: salt_str,
    };

    let mut anciennes_donnees: Vec<Entry> = match File::open(FICHIER_JSON) {
        Ok(mut fichier) => {
            let mut contenu = String::new();
            fichier.read_to_string(&mut contenu).unwrap_or(0);
            serde_json::from_str(&contenu).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    };

    anciennes_donnees.push(nouvelle_entree);

    let mut fichier = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FICHIER_JSON)
        .expect("Erreur lors de l'ouverture du fichier JSON");

    serde_json::to_writer_pretty(&mut fichier, &anciennes_donnees).is_ok()
}

// Fonction pour supprimer un mot de passe
fn supprimer_mdp(index: usize) -> bool {
    let mut fichier = match File::open(FICHIER_JSON) {
        Ok(fichier) => fichier,
        Err(_) => return false,
    };

    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu).unwrap_or(0);

    let mut entrees: Vec<Entry> = match serde_json::from_str(&contenu) {
        Ok(data) => data,
        Err(_) => return false,
    };

    if index >= entrees.len() {
        return false;
    }

    entrees.remove(index);

    let mut fichier = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FICHIER_JSON)
        .expect("Erreur lors de l'ouverture du fichier JSON");

    serde_json::to_writer_pretty(&mut fichier, &entrees).is_ok()
}
