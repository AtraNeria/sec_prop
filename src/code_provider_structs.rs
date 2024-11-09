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


/* TEST
pub fn ser_prov() {
    let one = ApiFunCall("fn_1_1".to_string(), 2);
    let two = ApiFunCall("fn_1_2".to_string(), 5);
    let three = ApiFunCall("fn_1_3".to_string(), 0);
    let provider_1 = CodeProvider {
        name: "provider_1".to_string(),
        api_name: "api_1".to_string(),
        offered_fn: vec![one,two,three],
        conflicts: None,
    };

    let two_one = ApiFunCall("fn_2_1".to_string(), 3);
    let two_two = ApiFunCall("fn_2_2".to_string(), 6);
    let two_third = ApiFunCall("fn_2_3".to_string(), 12);
    let two_four = ApiFunCall("fn_2_4".to_string(), 2);
    let provider_2 = CodeProvider {
        name: "provider_2".to_string(),
        api_name: "api_2".to_string(),
        offered_fn: vec![two_one, two_two, two_third, two_four],
        conflicts: Some(vec!["api_1".to_string()]),

    };

    let th_o = ApiFunCall("fn_3_1".to_string(), 8);
    let t_s = ApiFunCall("fn_3_2".to_string(), 1);
    let t_t = ApiFunCall("fn_3_3".to_string(), 13);
    let t_4 = ApiFunCall("fn_3_4".to_string(), 4);
    let provider_3 = CodeProvider {
        name: "provider_3".to_string(),
        api_name: "api_3".to_string(),
        offered_fn: vec![th_o,t_s,t_t,t_4],
        conflicts: Some(vec!["api_2".to_string()]),

    };

    let f_o = ApiFunCall("fn_2_1".to_string(), 7);
    let f_t = ApiFunCall("fn_2_2".to_string(), 2);
    let f_th = ApiFunCall("fn_2_3".to_string(), 1);
    let provider_4 = CodeProvider {
        name: "provider_4".to_string(),
        api_name: "api_4".to_string(),
        offered_fn: vec![f_o,f_t,f_th],
        conflicts: Some(vec!["api_1".to_string(),"api_2".to_string(),"api_3".to_string()]),
    };
    let providers = vec![provider_1,provider_2,provider_3,provider_4];
    let serialized = serde_json::to_string(&providers).unwrap();
    println!("{}",serialized);
}
*/

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