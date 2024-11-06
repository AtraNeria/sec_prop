use crate::automata_structs::Edge as Edge;
use crate::automata_structs::State as State;
use crate::code_provider_structs::CodeProvider as CodeProvider;
use crate::code_provider_structs::ApiFunCall as ApiFunCall;
use crate::code_provider_structs::get_providers;



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
fn test_api_automata (automa: Vec<State>, chosen_provider: CodeProvider, called: ApiFunCall) -> u8 {
    
    // Prelevo stato start
    let mut current_state = &automa[0];

    // Finchè non giungo ad uno stato finale
    while current_state.outgoing_edges.is_some() {

        let mut edge_found = false;
        // Risultato svolgimento funzione stato
        let action_res = match &current_state.action {
            Some(ar) => ar(chosen_provider.clone(),called.clone()),
            None => {return 4;}
        };

        // Controllo quale arco seguire
        for ed in current_state.outgoing_edges.as_ref().expect("REASON").iter() {
            // Se la condizione di un arco è rispettata
            if (ed.condition)(action_res.clone()) {
                // Se era già stato trovato un arco true
                // -> automa non deterministico
                if edge_found {return 1;}
                else {edge_found = true;}

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
                else {return 2};
            }
        }
    }
    // Se finisco in uno stato di fallimento
    // -> proprietà non rispettata
    if current_state.name.eq("Fail") {
        return 3;
    }
    // Altrimenti*/
    return 0;
}

// Print warning in base all'output di test_flow
fn print_result (res_code: u8) -> bool {
    match res_code {
        0 => {println!("End State");
            return true; 
        },
        1 => println!("Più di un edge true: automa non deterministico"),
        2 => println!("Stato non presente"),
        3 => println!("Fail State"),
        4 => println!("State Action not found"),
        _ => println!("Error"),
    }
    return false;
}

// Controlla se è presente conflitto tra il provider scelto e la chiamata di funzione
fn is_in_conflict (chosen_provider: CodeProvider, called: ApiFunCall) -> String {
    match chosen_provider.conflicts {
        Some(conf) =>{
            let providers_list = get_providers();
            let ApiFunCall(called_api_fun) = called;
            let mut result = "false".to_string();
            for prov in providers_list.iter() {
                if prov.offered_fn.contains(&called_api_fun) && conf.contains(&prov.api_name) {
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
    let ApiFunCall(called_api_fun) = called;
    if chosen_provider.offered_fn.contains(&called_api_fun) {
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

pub fn conflict_test (provider: CodeProvider, api_call: ApiFunCall) -> bool {
    let conflicts_aut = get_conflict_check_automata();    
    return print_result(test_api_automata(conflicts_aut, provider, api_call));
}

pub fn isolation_test (provider: CodeProvider, api_call: ApiFunCall) -> bool {
    let iso_aut = get_provider_isolation_automata();
    return print_result(test_api_automata(iso_aut, provider, api_call));
}