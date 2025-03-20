use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn save_mdp(mut utilisateur: String, mut mdp: String, mut lien: String){
    const FICHIER_JSON: &str = "./fichier.txt";
    
    let mut fichier = OpenOptions::new()
        .append(true)  // n'écrase pas les données précédentes
        .create(true)  // crée le fichier si il n'existe pas
        .open(FICHIER_JSON)
        .expect("Erreur lors de l'ouverture du fichier");

    println!("Enregistrement d'un mot de passe :\n");
    println!("Nom d'utilisateur: ");
    io::stdin().read_line(&mut utilisateur)
        .expect("Erreur lors de la lecture de l'entrée");
    writeln!(fichier, "utilisateur: {}", utilisateur.trim())
        .expect("Erreur lors de l'écriture dans le fichier");

    println!("Mot de passe: ");
    io::stdin().read_line(&mut mdp)
        .expect("Erreur lors de la lecture de l'entrée");
    writeln!(fichier, "mot de passe: {}", mdp.trim())
        .expect("Erreur lors de l'écriture dans le fichier");

    println!("Lien: ");
    io::stdin().read_line(&mut lien)
        .expect("Erreur lors de la lecture de l'entrée");
    writeln!(fichier, "lien: {}", lien.trim())
        .expect("Erreur lors de l'écriture dans le fichier");
    
    println!("Enregistrement fait avec succès!");
}

fn main() {
    let mut option = String::new();

    println!("Menu : \n1. Voir les mdp \n2. Enregistrer un mdp \n3. Supprimer un mdp \n\nChoisissez une option :");

    io::stdin().read_line(&mut option)
        .expect("Erreur lors de la lecture de l'entrée");

    //println!("option choisie: {}", option);

    //println!("{}", option);

    let option = option.trim();

    if option == "1" {
        println!("option 1");
    }
    else if option == "2" {
        let utilisateur = String::new();
        let mdp = String::new();
        let lien = String::new();
        save_mdp(utilisateur, mdp, lien);
    }
    else if option == "3" {
        println!("option 3");
    }
}
