use anyhow::Result;
use std::collections::BTreeSet as Set;
use std::collections::BTreeMap as Map;

fn help() {
    println!("Commandes: ");
    println!("voter <nom> <vote> : Voter pour quelqu'un");
    println!("votants : Afficher la liste des votants");
    println!("scores : fait afficher les scores pour tous les candidats");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Machine a vote: Que voulez-vous faire ?");

    let mut voters: Set<String> = Set::new();

    let mut scores = Map::new();
    let white_string = String::from("Blanc");
    let null_string = String::from("Nulles");

    scores.insert(white_string, 0);
    scores.insert(null_string, 0);

    let mut candidates = Set::new();
    candidates.insert("Linux");

    for candidate in candidates.iter() {
        scores.insert(candidate.to_string(), 0);
    }

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.clone();
        let input_args = input.split_whitespace().collect::<Vec<&str>>();

        if input_args.is_empty() {
            help();
        } else {
            if input_args.len() == 0 {
                println!("La commande n'existe pas");
            } else {
                let command = input_args[0];
                
                if command == "voter" {
                    if input_args.len() < 2 {
                        println!("Vous devez utiliser la commande de la façon suivante : voter <nom> <vote>");
                        continue;
                    }
    
                    let name = input_args[1].to_string();
    
                    if voters.contains(&name) {
                        println!("Vous avez déjà voter !");
                        continue;
                    }
                    
                    voters.insert(name.clone());
    
                    if input_args.len() == 2 {
                        scores.get_mut("Blanc").map(|score| *score += 1);
                        println!("{} a voté pour Blanc", name, );
                    } else {
                        let vote = input_args[2];
    
                        if !candidates.contains(vote) {
                            scores.get_mut("Nulles").map(|score| *score += 1);
    
                            println!("{} a voté nul", name);
                        } else {
                            if scores.contains_key(vote) {
                                scores.get_mut(vote).map(|score| *score += 1);
                            } else {
                                scores.insert(vote.to_string(), 1);
                            }
                            
                            println!("{} a voté pour {}", name, vote);
                        }
    
                    }
                } 
                else if command == "votants" {
                    println!("Liste des votants: ");
                    for element in voters.iter() {
                        println!("{}", element);
                    }
                    
                }
                else if command == "scores" {
                    println!("Scores: ");
                    for (key, value) in scores.iter() {
                        println!("{}: {}", key, value);
                    }
    
                }
                else {
                    println!("Commande invalide");
                }
            }

        }
    }
}
