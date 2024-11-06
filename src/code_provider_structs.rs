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
    pub offered_fn: Vec<String>,
    // Provider con cui è in conflitto
    pub conflicts: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct ApiFunCall(pub String);

// TEST
/*pub fn ser_prov() {
    let provider_1 = CodeProvider {
        name: "provider_1".to_string(),
        api_name: "api_1".to_string(),
        offered_fn: vec!["fn_1_1".to_string(), "fn_1_2".to_string(), "fn_1_3".to_string()],
        conflicts: None,
    };
    let provider_2 = CodeProvider {
        name: "provider_2".to_string(),
        api_name: "api_2".to_string(),
        offered_fn: vec!["fn_2_1".to_string(), "fn_2_2".to_string(), "fn_2_3".to_string(), "fn_2_4".to_string()],
        conflicts: Some(vec!["api_1".to_string()]),

    };
    let provider_3 = CodeProvider {
        name: "provider_3".to_string(),
        api_name: "api_3".to_string(),
        offered_fn: vec!["fn_3_1".to_string(), "fn_3_2".to_string(), "fn_3_3".to_string(), "fn_3_4".to_string()],
        conflicts: Some(vec!["api_2".to_string()]),

    };
    let provider_4 = CodeProvider {
        name: "provider_4".to_string(),
        api_name: "api_4".to_string(),
        offered_fn: vec!["fn_4_1".to_string(), "fn_4_2".to_string()],
        conflicts: Some(vec!["api_1".to_string(),"api_2".to_string(),"api_3".to_string()]),
    };
    let providers = vec![provider_1,provider_2,provider_3,provider_4];
    let serialized = serde_json::to_string(&providers).unwrap();
    println!("{}",serialized);
}*/

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
pub fn get_api_fns () -> Vec<String> {
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
        return ApiFunCall(funs[usize::arbitrary(g)%funs.len()].clone());
    }
}