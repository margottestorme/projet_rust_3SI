use eframe::egui;
use crate::chiffrement::{chiffrer_donnee, dechiffrer_donnee, derive_key, generate_salt};
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read};

const FICHIER_JSON: &str = "./fichier.json";

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    utilisateur: String,
    username: String,
    mdp: String,
    lien: String,
    nonce: String,
    salt: String,
}

pub struct MonApp {
    logged_in: bool,
    username: String,
    password: String,
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
            username: String::new(),
            password: String::new(),
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
                            if enregistrer_mdp(&self.site, &self.username, &self.lien, &self.mot_de_passe) {
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
                                        let master_password = "master_password";
                                        let salt = general_purpose::STANDARD
                                            .decode(&entry.salt)
                                            .expect("Erreur de décodage du salt");
                                        let cle = derive_key(master_password, &salt);
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
            } else {
                // Page de connexion
                ui.heading("Connexion");
                ui.horizontal(|ui| {
                    ui.label("Nom d'utilisateur :");
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Mot de passe :");
                    ui.text_edit_singleline(&mut self.password);
                });
                if ui.button("Se connecter").clicked() {
                    if self.username == "admin" && self.password == "password" {
                        self.logged_in = true;
                    } else {
                        ui.label("Nom d'utilisateur ou mot de passe incorrect.");
                    }
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

    let entrees: Vec<Entry> = serde_json::from_str(&contenu).map_err(|_| "Format JSON invalide.")?;

    Ok(entrees)
}

// Fonction pour enregistrer un mot de passe
fn enregistrer_mdp(site: &str, username: &str, lien: &str, mot_de_passe: &str) -> bool {
    let master_password = "master_password"; // a remplacer plus tard par une méthode + secu
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