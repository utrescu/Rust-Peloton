use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::vec::Vec;

struct Corredor {
    temps: String,
    posicio: u16,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "in.txt";

    if args.len() > 2 {
        panic!("Please enter ONE filename as a param");
    }

    if args.len() == 2 {
        filename = &args[1];
    }

    let mut quants_corredors_queden: i16 = -1;

    let file = File::open(filename).unwrap();
    let mut corredors: Vec<Corredor> = Vec::new();
    for line in BufReader::new(file).lines() {
        if quants_corredors_queden == -1 {
            quants_corredors_queden = line.unwrap().to_string().parse().unwrap();
        } else if quants_corredors_queden == 0 {
            // Acabat d'agafar els corredors, toca calcular la seva posició
            corredors = calcular_resultat(&corredors);
            imprimir_resultat(&corredors);
            // Mirem el següent
            quants_corredors_queden = line.unwrap().to_string().parse().unwrap();
            println!("---");
            corredors.clear();
        } else {
            let valor = line.unwrap().to_string();
            corredors.push(Corredor {
                temps: valor,
                posicio: 0,
            });
            quants_corredors_queden = quants_corredors_queden - 1;
        }
    }
    // El darrer no s'ha imprimit
    if quants_corredors_queden != -1 {
        corredors = calcular_resultat(&corredors);
        imprimir_resultat(&corredors);
    }
}

/**
 * Se n'encarrega de generar les posicions de cada un dels corredors de la llista.
 */
fn calcular_resultat(corredors: &Vec<Corredor>) -> Vec<Corredor> {
    let mut resultats: Vec<Corredor> = vec![];
    for corredor in corredors.iter() {
        resultats.push(Corredor {
            temps: corredor.temps.clone(),
            posicio: corredor.posicio,
        });
    }

    let mut tempus: Vec<String> = corredors.iter().map(|m| m.temps.clone()).collect();
    tempus.sort();
    tempus.dedup();

    let mut posicio: u16 = 1;
    let mut posats: u16 = 1;
    let valor = calculasegons(&tempus[0]);
    let mut hora_anterior = Duration::from_secs(valor);

    for tempu in tempus.iter() {
        let valor = calculasegons(&tempu);
        let mut hora_arribada = Duration::from_secs(valor);
        let elapsed = hora_arribada - hora_anterior;
        if elapsed.as_secs() > 1 {
            posicio = posats;
        }

        hora_anterior = hora_arribada;

        let temps_actual: String = tempu.to_string();

        resultats.iter_mut().for_each(|el| {
            if el.temps == temps_actual {
                el.posicio = posicio;
                posats = posats + 1;
            }
        });
    }

    resultats
}

/**
 * Imprimeix els resultats
 */
fn imprimir_resultat(corredors: &Vec<Corredor>) {
    for corredor in corredors.iter() {
        println!("{} - {}", corredor.temps, corredor.posicio);
    }

    // println!("{:?}", corredors);
}

/**
 * Com que Duration va amb segons he de calcular quants
 * segons té el temps definit
 */
fn calculasegons(temps: &String) -> u64 {
    let mut segons: u64 = 0;
    let mut multiplicador = 60 * 60;
    let split = temps.split(":");
    for s in split {
        let nous = s.parse::<u64>().unwrap() * multiplicador;
        segons = segons + nous;
        multiplicador = multiplicador / 60;
    }
    segons
}
