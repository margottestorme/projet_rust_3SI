use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use serde_json;

const FICHIER_JSON: &str = "./fichier.json";

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    utilisateur: String,
    mdp: String,
    lien: String,
}

fn save_mdp() {
    println!("Enregistrement d'un mot de passe :\n");

    let mut utilisateur = String::new();
    let mut mdp = String::new();
    let mut lien = String::new();

    println!("Nom d'utilisateur: ");
    io::stdin().read_line(&mut utilisateur).expect("Erreur lors de la lecture de l'entrée");

    println!("Mot de passe: ");
    io::stdin().read_line(&mut mdp).expect("Erreur lors de la lecture de l'entrée");

    println!("Lien: ");
    io::stdin().read_line(&mut lien).expect("Erreur lors de la lecture de l'entrée");

    // créer une nouvelle entrée JSON
    let nouvelle_entree = Entry {
        utilisateur: utilisateur.trim().to_string(),
        mdp: mdp.trim().to_string(),
        lien: lien.trim().to_string(),
    };

    // lire les anciennes données du fichier JSON
    let mut anciennes_donnees: Vec<Entry> = match File::open(FICHIER_JSON) {
        Ok(mut fichier) => {
            let mut contenu = String::new();
            fichier.read_to_string(&mut contenu).unwrap_or(0);
            serde_json::from_str(&contenu).unwrap_or_else(|_| Vec::new()) // retourne un tableau vide si y'a une erreur
        }
        Err(_) => Vec::new(), // tableau vide si le fichier n'existe pas
    };

    // ajout de la nouvelle entrée
    anciennes_donnees.push(nouvelle_entree);

    // conversion en JSON
    let json_string = serde_json::to_string_pretty(&anciennes_donnees).expect("Erreur de conversion en JSON");

    // sauvegarde dans le fichier
    let mut fichier = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Efface l'ancien contenu pour réécrire un JSON propre
        .open(FICHIER_JSON)
        .expect("Erreur lors de l'ouverture du fichier");

    fichier.write_all(json_string.as_bytes()).expect("Erreur lors de l'écriture dans le fichier");

    println!("Enregistrement fait avec succès !");
}

fn display_mdp() {
    // ouvrir le fichier en lecture seule
    let fichier = File::open(FICHIER_JSON);

    match fichier {
        Ok(mut fichier) => {
            let mut contenu = String::new();
            fichier.read_to_string(&mut contenu).expect("Erreur de lecture du fichier");

            if contenu.trim().is_empty() {
                println!("Aucun mot de passe enregistré.");
                return;
            }

            // Convertir le JSON en structure Rust
            let entrees: Vec<Entry> = match serde_json::from_str(&contenu) {
                Ok(data) => data,
                Err(_) => {
                    println!("Erreur de lecture du JSON.");
                    return;
                }
            };

            println!("Liste des mots de passe enregistrés :\n");
            for entree in entrees {
                println!("Utilisateur : {}", entree.utilisateur);
                println!("Mot de passe : {}", entree.mdp);
                println!("Lien : {}", entree.lien);
            }
        }
        Err(_) => {
            println!("Aucun fichier trouvé. Aucun mot de passe enregistré.");
        }
    }
}

fn main() {
    let mut option = String::new();

    println!("Menu : \n1. Voir les mdp \n2. Enregistrer un mdp \n3. Supprimer un mdp \n\nChoisissez une option :");

    io::stdin().read_line(&mut option)
        .expect("Erreur lors de la lecture de l'entrée");

    let option = option.trim();

    if option == "1" {
        display_mdp();
    }
    else if option == "2" {
        save_mdp();
    }
    else if option == "3" {
        println!("option 3");
    }
}
