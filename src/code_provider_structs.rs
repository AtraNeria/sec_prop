use serde::{Deserialize, Serialize};
use quickcheck::{Arbitrary,Gen};
use std::fs::File;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CodeProvider {
    // Nome del provider
    pub name: String,
    // Nome dell'ap offerta
    pub api_name: String,
    // Nomi delle funzioni offerte dall'API
    pub offered_fn: Vec<ApiFunCall>,
    // Provider con cui è in conflitto
    pub conflicts: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApiFunCall {
    pub name: String, 
    pub connections_required: u8
}

#[derive(Clone, Debug)]
pub struct MaxConnections (pub u8);

#[derive(Clone, Debug)]
pub struct FunGroup (pub Vec<ApiFunCall>);


// Restituisce tutti i providers di API registrati sul file dummy_providers.json
pub fn get_providers () -> Vec<CodeProvider> {
    let f = File::open("dummy_providers.json");
    match f {
        Ok(file) => {
            let providers_list : Vec<CodeProvider> = serde_json::from_reader(file).unwrap();
            return providers_list;
        },
        Err(errore) => panic!("Errore nel file che contiene la lista dei code providers!\n{}",errore),
    };
}

// Restituisce la lista delle funzioni offerte dalle API
pub fn get_api_fns () -> Vec<ApiFunCall> {
    let providers = get_providers();
    let mut fns = vec![];
    for prov in providers.iter() {
        fns.extend(prov.offered_fn.clone());
    }
    return fns;
}

// Implementazione Arbitrary
// Per estrarre casualmente un code provider da utilizzare
impl Arbitrary for CodeProvider {
    fn arbitrary(g: &mut Gen) -> CodeProvider {
        let options = get_providers();
        return options[usize::arbitrary(g)%options.len()].clone();
    }
}

// Estrazione casuale di una chiamata di funzione dalle API
impl Arbitrary for ApiFunCall {
    fn arbitrary(g: &mut Gen) -> ApiFunCall {
        let funs = get_api_fns();
        return funs[usize::arbitrary(g)%funs.len()].clone();
    }
}

// Estrazione casuale di numero di connessioni massime
impl Arbitrary for MaxConnections {
    fn arbitrary(g: &mut Gen) -> MaxConnections {
        let possible_funs = get_api_fns();
        let mut max_required = 0;
        for f in possible_funs.iter() {
            max_required = max_required + f.connections_required;
        }
        return MaxConnections((u8::arbitrary(g)%max_required)+1);
    }
}

// Estrazione casuale di un gruppo di funzioni
impl Arbitrary for FunGroup {
    fn arbitrary(g: &mut Gen) -> FunGroup {
        let possible_funs = get_api_fns();
        let called = (usize::arbitrary(g) % possible_funs.len())+1;
        let mut fns = vec![];
        for _i in 1..called {
            fns.push(ApiFunCall::arbitrary(g));
        }
        return FunGroup(fns);
    }
}