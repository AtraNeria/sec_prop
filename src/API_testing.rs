use crate::automata_structs::Edge as Edge;
use crate::automata_structs::State as State;
use nix::unistd;
use is_root::is_root;


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
fn test_api_automata (automa: Vec<State>) -> u8 {
    
    // Prelevo stato start
    let mut current_state = &automa[0];

    // Finchè non giungo ad uno stato finale
    while current_state.outgoing_edges.is_some() {

        let mut edge_found = false;
        // Risultato svolgimento funzione stato

        let action_res = match &current_state.action {
            Some(ar) => ar(),
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



fn root_check () -> String {
    if is_root() {return "true".to_string();}
    else {
        println!("{}",nix::unistd::getuid()); // TEST
        return "false".to_string();}
}

// Costruttore per automa prima politica
// Prima di poter utilizzare funzioni di terze parti si richiede controllo credenziali
fn get_root_check_automata () -> Vec<State> {

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
        action: Some(Box::new(root_check)),
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


fn not_written () -> String {
    // TO-DO
    return "true".to_string();
}

// Costruttore per automa seconda politica
// Call a funzione di terze parti non può effettuare modifiche
fn get_no_writing_automata () -> Vec<State> {

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
        action: Some(Box::new(not_written)),
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

fn no_more_call () -> String {
    // TO-DO
    return "true".to_string();
}

// Costruttore per automa terza politica
// Si può chiamare una singola funzione di terze parti
fn get_single_call_automata () -> Vec<State> {

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
        action: Some(Box::new(no_more_call)),
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


// First policy test 
pub fn root_check_test () -> bool {
    let cc_automata = get_root_check_automata();
    return print_result(test_api_automata(cc_automata));
}

pub fn no_writing_test () -> bool {
    let nw_automata = get_no_writing_automata();
    let res = test_api_automata(nw_automata);
    let mut ret_value = false;
    match res {
        0 => {println!("End State");
            ret_value = true; },
        1 => println!("Più di un edge true: automa non deterministico"),
        2 => println!("Stato non presente"),
        3 => println!("Fail State"),
        4 => println!("State Action not found"),
        _ => println!("Error"),
    }
    return ret_value;
}

pub fn single_call_test () -> bool {
    let sc_automata = get_single_call_automata();
    let res = test_api_automata(sc_automata);
    let mut ret_value = false;
    match res {
        0 => {println!("End State");
            ret_value = true; },
        1 => println!("Più di un edge true: automa non deterministico"),
        2 => println!("Stato non presente"),
        3 => println!("Fail State"),
        4 => println!("State Action not found"),
        _ => println!("Error"),
    }
    return ret_value;
}