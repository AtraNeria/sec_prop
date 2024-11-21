use crate::automata_structs::{Edge, State, Result, EdgeId};

use crate::operations_structs::OpFlow as OpFlow;

// Checks if the operation is "Read"
fn op_is_read(op: String)-> bool {
    if op.eq("RD") {
        return true;
    }
    else {
        return false;
    }
}

// Checks if the operation is not "Read"
fn op_is_not_read(op: String)-> bool {
    if op.eq("RD") {
        return false;
    }
    else {
        return true;
    }
}

// Checks if the operation is "Write"
fn op_is_write(op: String)-> bool {
    if op.eq("WR") {
        return true;
    }
    else {
        return false;
    }
}

// Checks if the operation is not "Write"
fn op_is_not_write(op: String)-> bool {
    if op.eq("WR") {
        return false;
    }
    else {
        return true;
    }
}

// Checks if operation is open
fn op_is_open(op: String)-> bool {
    if op.eq("OP") {
        return true;
    }
    else {
        return false;
    }
}

// Checks if operation is NOT open
fn op_is_not_open(op: String)-> bool {
    if op.eq("OP") {
        return false;
    }
    else {
        return true;
    }
}

// Checks if operation is create
fn op_is_create(op: String)-> bool {
    if op.eq("CR") {
        return true;
    }
    else {
        return false;
    }
}

// Checks if operation is different from either open or create
fn op_is_not_opencreate(op: String)-> bool {
    if op.eq("CR") || op.eq("OP") {
        return false;
    }
    else {
        return true;
    }
}




// Testo il flusso di operazioni ops sull'automa
fn test_flow (ops: OpFlow, automa: Vec<State>) -> Result {
    // Prelevo stato start
    let mut current_state = &automa[0];
    // Destruct OpFlow
    let OpFlow(mut op_seq) = ops;
    // Prelevo prima operazione richiesta    
    let mut curr_op = op_seq.pop();

    // Inizializzo risultato del test
    let mut res = Result {
        result_code: 0,
        explored_states: 0,
        next_state_unfound: None,
        true_edges: None,
    };

    // Finchè non giungo ad uno stato finale
    while current_state.outgoing_edges.is_some() {

        let mut edge_found = false;
        // Controllo quale arco seguire
        for ed in current_state.outgoing_edges.as_ref().expect("REASON").iter() {

            res.explored_states = res.explored_states + 1;
            // Se la condizione di un arco è rispettata
            if (ed.condition)(curr_op.as_ref().expect("REASON").to_string()) {
                
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

                let mut next_state = current_state;
                let mut state_index = automa.iter();

                // Cerco prossimo stato nell'automa
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
        //Estraggo prossima operazione
        curr_op = op_seq.pop();
    }

    // Se finisco in uno stato di fallimento
    // -> proprietà non rispettata
    if current_state.name.eq("Fail") {
        res.result_code = 3;
    }
    // Altrimenti
    return res;
}

// Print warning in base all'output di test_flow
fn print_result (result: Result, ops: OpFlow) -> bool {
    match result.result_code {
        // Stato finale
        0 => {
            println!("End State");
            return true; 
        },
        // Automa non deterministico
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
            println!("Raggiunto stato di fallimento.\nLa sequenza di operazioni che ha fallito è:");
            let OpFlow(ref op_seq) = ops;
            for i in (op_seq.len()-(result.explored_states as usize)-1..op_seq.len()).rev() {
                match op_seq[i as usize].as_str() {
                    "RD" => println!("Read"),
                    "WR" => println!("Write"),
                    "CR" => println!("Create"),
                    "CL" => println!("Close"),
                    "OP" => println!("Open"),
                    _ => println!("Errore: operazione non prevista"),
                }
            }
        },
        _ => println!("Error"),
    }
    return false;
}

// Costruttore per automa che rappresenta la prima politica
// Prima di poter eseguire un'operazione su un file questo deve essere aperto
fn get_open_first_automata () -> Vec<State> {

    //First State: start
    let one_to_two = Edge {
        condition: Box::new(op_is_create),
        where_to: "file_created".to_string(),
    };

    let one_to_three = Edge {
        condition: Box::new(op_is_open),
        where_to: "file_opened".to_string(),
    };

    let one_to_fail = Edge {
        condition: Box::new(op_is_not_opencreate), 
        where_to: "Fail".to_string(),
    };

    let start_state = State {
        name: "Start".to_string(),
        action: None,
        is_starting: true,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![one_to_two, one_to_three, one_to_fail]),
    };    

    //Second: created
    let two_to_three = Edge {
        condition: Box::new(op_is_open),
        where_to: "file_opened".to_string(),
    };

    let two_to_fail = Edge {
        condition: Box::new(op_is_not_open), 
        where_to: "Fail".to_string(),

    };

    let second_state = State {
        name: "file_created".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![two_to_three, two_to_fail]),
    };    

    //Third: opened
    let three_to_end = Edge {
        condition: Box::new(op_is_not_open),
        where_to: "End".to_string(),
    };

    let three_to_fail = Edge {
        condition: Box::new(op_is_open), 
        where_to: "Fail".to_string(),

    };

    let third_state = State {
        name: "file_opened".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![three_to_end, three_to_fail]),
    };

    //End State: operation permitted
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
        // Both true: facile da modificare per inserire un flusso in cui si risponde al fallimento
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state,second_state,third_state,end_state,fail_state]
}

// Costruttore per automa che rappresenta la seconda politica
// File read-only
fn get_read_only_automata () -> Vec<State> {

    // First State: start
    let one_to_two = Edge {
        condition: Box::new(op_is_open),
        where_to: "file_opened".to_string(),
    };

    let one_to_fail = Edge {
        condition: Box::new(op_is_not_open), 
        where_to: "Fail".to_string(),
    };

    let start_state = State {
        name: "Start".to_string(),
        action: None,
        is_starting: true,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![one_to_two, one_to_fail]),
    };

    // Second State: file opened
    let two_to_end = Edge {
        condition: Box::new(op_is_read),
        where_to: "End".to_string(),
    };

    let two_to_fail = Edge {
        condition: Box::new(op_is_not_read), 
        where_to: "Fail".to_string(),

    };

    let second_state = State {
        name: "file_opened".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![two_to_end, two_to_fail]),
    };

    //End State: operation permitted
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
        // Both true: facile da modificare per inserire un flusso in cui si risponde al fallimento
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state,second_state,end_state,fail_state]

}

// Costruttore per automa che rappresenta la terza politica
// Singola modifica in write permessa
fn get_single_write_automata () -> Vec<State> {
    
    // First state: start
    let one_to_two = Edge {
        condition: Box::new(op_is_create),
        where_to: "file_created".to_string(),
    };
    let one_to_three = Edge {
        condition: Box::new(op_is_open),
        where_to: "file_opened".to_string(),
    };
    let one_to_fail = Edge {
        condition: Box::new(op_is_not_opencreate),
        where_to: "Fail".to_string(),
    };
    let start_state = State {
        name: "Start".to_string(),
        action: None,
        is_starting: true,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![one_to_two, one_to_three, one_to_fail]),
    };

    // Second state: file created
    let two_to_three = Edge {
        condition: Box::new(op_is_open),
        where_to: "file_opened".to_string(),
    };
    let two_to_fail = Edge {
        condition: Box::new(op_is_not_opencreate), 
        where_to: "Fail".to_string(),

    };
    let two_to_end = Edge {
        condition: Box::new(op_is_create),
        where_to: "End".to_string(),
    };
    let second_state = State {
        name: "file_created".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![two_to_three, two_to_fail, two_to_end]),
    };    

    // Third: opened
    let three_to_end = Edge {
        condition: Box::new(op_is_not_write),
        where_to: "End".to_string(),
    };
    let three_to_four = Edge {
        condition: Box::new(op_is_write), 
        where_to: "file_written".to_string(),

    };
    let third_state = State {
        name: "file_opened".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![three_to_end, three_to_four]),
    };

    // Fourth: written
    let three_to_end = Edge {
        condition: Box::new(op_is_not_write),
        where_to: "End".to_string(),
    };
    let three_to_fail = Edge {
        condition: Box::new(op_is_write), 
        where_to: "Fail".to_string(),

    };
    let fourth_state = State {
        name: "file_written".to_string(),
        action: None,
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![three_to_end, three_to_fail]),
    };

    //End State: operation permitted
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
        // Both true: facile da modificare per inserire un flusso in cui si risponde al fallimento
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state,second_state,third_state,fourth_state,end_state,fail_state]

}

// Test prima proprietà
pub fn open_first(ops:OpFlow) -> bool {
    // Creo automa e testo ops
    let op_first_auto : Vec<State> = get_open_first_automata();
    return print_result(test_flow(ops.clone(), op_first_auto),ops);
}

// Test seconda proprietà
pub fn read_only(ops:OpFlow) -> bool {
    let read_only_auto = get_read_only_automata();
    return print_result(test_flow(ops.clone(), read_only_auto),ops);
}

// Test terza proprietà
pub fn single_write(ops:OpFlow) -> bool {
    let single_write_auto = get_single_write_automata();
    return print_result(test_flow(ops.clone(), single_write_auto),ops);
}