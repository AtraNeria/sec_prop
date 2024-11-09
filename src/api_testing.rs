use crate::automata_structs::{Edge, State, TestResult, EdgeId};
use crate::code_provider_structs::{CodeProvider, ApiFunCall, FunGroup, MaxConnections, get_providers};

// Struttura per contenere il risultato del test sulle connessioni
struct ConnectionResult {
    result_code: u8,
    functions_called: u8,
    finale_cons: u8,
}


fn is_true(res: String)-> bool {
    if res.eq("true") {
        return true;
    }
    else {
        return false;
    }
}

fn is_false(res: String)-> bool {
    if res.eq("false") {
        return true;
    }
    else {
        return false;
    }
}


// Funzione per il testing su automa proprietà su API
fn test_api_automata (automa: Vec<State>, chosen_provider: CodeProvider, called: ApiFunCall) -> TestResult {
    // Inizializzo risultato del test
    let mut res = TestResult {
        result_code: 0,
        explored_states: 0,
        next_state_unfound: None,
        true_edges: None,
    };
    
    // Prelevo stato start
    let mut current_state = &automa[0];

    // Finchè non giungo ad uno stato finale
    while current_state.outgoing_edges.is_some() {

        let mut edge_found = false;
        // Risultato svolgimento funzione stato
        let action_res = match &current_state.action {
            Some(ar) => ar(chosen_provider.clone(),called.clone()),
            None => {res.result_code = 3; 
                return res;},
        };

        // Controllo quale arco seguire
        for ed in current_state.outgoing_edges.as_ref().expect("REASON").iter() {
            // Se la condizione di un arco è rispettata
            if (ed.condition)(action_res.clone()) {
                // Se era già stato trovato un arco true
                // -> automa non deterministico
                if edge_found {
                    res.result_code = 1;
                    let curr_edge_id = EdgeId {
                        from_state : current_state.name.clone(),
                        to_state : ed.where_to.clone(),
                    };
                    res.true_edges.as_mut().expect("REASON").push(curr_edge_id);
                    return res;
                }                
                else {
                    edge_found = true;
                    let curr_edge_id = EdgeId {
                        from_state : current_state.name.clone(),
                        to_state : ed.where_to.clone(),
                    };
                    res.true_edges = Some(vec![curr_edge_id]);
                }

                // Cerco prossimo stato nell'automa
                let mut next_state = current_state;
                let mut state_index = automa.iter();
                while next_state.name.ne(&ed.where_to) {
                    next_state = state_index.next().expect("REASON");
                }

                // Stato trovato
                if next_state.name.eq(&ed.where_to) {
                    current_state = next_state;
                }
                // Stato non presente
                else {
                    res.result_code = 2;
                    res.next_state_unfound = Some(ed.where_to.clone());
                    return res;
                }
            }
        }
    }
    // Se finisco in uno stato di fallimento
    // -> proprietà non rispettata
    if current_state.name.eq("Fail") {
        res.result_code = 3;
    }
    // Altrimenti
    return res;
}

// Funzione per stampare i risultati dei test su conflitti e isolamento
fn print_result (result: TestResult, chosen_provider: CodeProvider, called: ApiFunCall, fn_tested: u8) -> bool {
    match result.result_code {
        0 => {
            println!("Stato finale: successo");
            return true;
        }
        1 => {
            println!("Automa non deterministico: rilevati più archi possibili da seguire");
            match result.true_edges {
                Some(edges) => for edge_id in edges {
                    println!("Arco da {} a {}",edge_id.from_state,edge_id.to_state);
                },
                None => println!("Errore"),
            }
        },
        // Stato non trovato
        2 => {
            let nsu = result.next_state_unfound;
            match nsu {
                Some(nsu) => println!{"Stato non presente \nLo stato {} non è stato trovato",nsu},
                None => println!("Stato non presente"),
            };
        },
        // Stato di fallimento
        3 => {
            if fn_tested==0 {   
                println!("Fallimento: la funzione {} è in conflitto con il fornitore {}",called.name, chosen_provider.name);}
            else {
                println!("Fallimento: la funzione {} non appartiene al provider {}", called.name, chosen_provider.name);
            }
        },
        4 => println!("Operazione dello stato da svolgere non presente"),
        _ => println!("Errore non previsto"),

    }
    return false;
}

// Funzione per stampare i risultati del test sulle connessioni
fn print_connections_result (result: ConnectionResult, con_max: MaxConnections, api_calls: FunGroup) -> bool {
    let MaxConnections(mc) = con_max;
    match result.result_code {
        0 => {
            println!("Tutte le funzioni richieste sono state chiamate! Si sono usate {} su {} connessioni disponibili", 
            result.finale_cons,
            mc);
            return true;
        },
        3 => {
            println!("Test fallito! Si sono chiamate le funzioni:");
            let FunGroup(fg) = api_calls;
            for i in (fg.len()-(result.functions_called as usize)-1.. fg.len()).rev() {
                println!("{} con connessioni richieste {}",fg[i].name, fg[i].connections_required);
            }
            println!("Per un totale di {} su {} connessioni disponibili",result.finale_cons, mc);
        },
        _ => println!("Errore non previsto"),

    }
    return false;
}

// Controlla se provider offre la funzione func
fn offers_fun (provider: CodeProvider, func: ApiFunCall) -> bool {
    for offered in provider.offered_fn.iter() {
        if offered.name.eq(&func.name) {
            return true;
        }
    }
    return false;
}

// Controlla se è presente conflitto tra il provider scelto e la chiamata di funzione
fn is_in_conflict (chosen_provider: CodeProvider, called: ApiFunCall) -> String {
    match chosen_provider.conflicts {
        Some(conf) =>{
            let providers_list = get_providers();
            let mut result = "false".to_string();
            for prov in providers_list.iter() {
                if offers_fun(prov.clone(), called.clone()) && conf.contains(&prov.api_name) {
                    result = "true".to_string();}
            }
            return result;
        },
        None => return "false".to_string(),
    };
}

// Restituisce automa di controllo dei conflitti
fn get_conflict_check_automata () -> Vec<State> {
    // Stato iniziale
    let one_to_end = Edge {
        condition: Box::new(is_false),
        where_to: "End".to_string(),
    };
    let one_to_fail = Edge {
        condition: Box::new(is_true),
        where_to: "Fail".to_string(),
    };
    let start_state = State {
        name: "Start".to_string(),
        action: Some(Box::new(is_in_conflict)),
        is_starting: true,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![one_to_end, one_to_fail]),
    };

    //End State
    let end_state = State {
        name: "End".to_string(),
        action: None,
        is_starting: false,
        is_terminal: true,
        is_failure: false,
        outgoing_edges: None,
    };

    //Fail State
    let fail_state = State {
        name: "Fail".to_string(),
        action: None,
        is_starting: false,
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state, end_state, fail_state]
}

// Controlla se la chiamata mantiene il provider isolato
fn isolation_check (chosen_provider: CodeProvider, called: ApiFunCall) -> String {
    if offers_fun(chosen_provider, called) {
        return "true".to_string();
    }
    else {return "false".to_string();}
}

// Restituisce automa di controllo dell'isolamento
fn get_provider_isolation_automata () -> Vec<State> {
    // Stato iniziale
    let one_to_end = Edge {
        condition: Box::new(is_true),
        where_to: "End".to_string(),
    };
    let one_to_fail = Edge {
        condition: Box::new(is_false),
        where_to: "Fail".to_string(),
    };
    let start_state = State {
        name: "Start".to_string(),
        action: Some(Box::new(isolation_check)),
        is_starting: true,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![one_to_end, one_to_fail]),
    };

    //End State
    let end_state = State {
        name: "End".to_string(),
        action: None,
        is_starting: false,
        is_terminal: true,
        is_failure: false,
        outgoing_edges: None,
    };

    //Fail State
    let fail_state = State {
        name: "Fail".to_string(),
        action: None,
        is_starting: false,
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state, end_state, fail_state]
}

// Funzione per testare automa di numero connessioni
fn test_connections (max_con: MaxConnections, called: FunGroup, curr_con: u8, funs_called: u8) -> ConnectionResult {
    let FunGroup(mut api_calls) = called;
    let MaxConnections(mc) = max_con;
    let next_call = api_calls.pop();
    
    match next_call {
        // Se ci sono ancora funzioni da chiamare
        Some(call) => {
            let new_con = curr_con + call.connections_required;
            let new_called = funs_called +1;
            // Se si è superato il limite delle connessioni
            if new_con > mc {
                let res = ConnectionResult {
                    result_code: 3,
                    functions_called: new_called,
                    finale_cons: new_con,
                };
                return res;
            }
            else {return test_connections(max_con, FunGroup(api_calls), new_con, new_called);}
        },
        // Se sono state chiamate tutte
        None => {
            let res = ConnectionResult {
                result_code: 0,
                functions_called: funs_called,
                finale_cons: curr_con,
            };
            return res;
        },
    }
}

// Da passare a quickcheck per il test su conflitti
pub fn conflict_test (provider: CodeProvider, api_call: ApiFunCall) -> bool {
    let conflicts_aut = get_conflict_check_automata();    
    return print_result(test_api_automata(conflicts_aut, provider.clone(), api_call.clone()), provider, api_call, 0);
}

// Da passare a quickcheck per il test su isolamento
pub fn isolation_test (provider: CodeProvider, api_call: ApiFunCall) -> bool {
    let iso_aut = get_provider_isolation_automata();
    return print_result(test_api_automata(iso_aut, provider.clone(), api_call.clone()), provider, api_call, 1);
}

// Da passare a quickcheck per il test su connessioni
pub fn connections_test (con_max: MaxConnections, api_calls: FunGroup) -> bool {
    return print_connections_result(test_connections(con_max.clone(), api_calls.clone(), 0, 0), con_max, api_calls);
}