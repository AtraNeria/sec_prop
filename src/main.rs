use quickcheck::quickcheck;

use automata_structs::Edge as Edge;
use automata_structs::State as State;
mod automata_structs;

use operations_structs::OpFlow as OpFlow;
mod operations_structs;

// Checks if the operation is "Read"
fn _op_is_read(op: String)-> bool {
    if op.eq("read") {
        return true;
    }
    else {
        return false;
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

//TEST for the starting state
fn start() {
    println!("Start point of the automata");
}

// Costruttore per automa che rappresenta la prima politica
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
        action: start, //TO_DO
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
        action: start, //TO_DO
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
        action: start, //TO_DO
        is_starting: false,
        is_terminal: false,
        is_failure: false,
        outgoing_edges: Some(vec![three_to_end, three_to_fail]),
    };

    //End State: operation permitted
    let end_state = State {
        name: "End".to_string(),
        action: start, //TO_DO
        is_starting: false,
        is_terminal: true,
        is_failure: false,
        outgoing_edges: None,
    };

    //Fail State
    let fail_state = State {
        name: "Fail".to_string(),
        action: start, //TO_DO
        is_starting: false,
        // Both true: facile da modificare per inserire un flusso in cui si risponde al fallimento
        is_terminal: true,
        is_failure: true,
        outgoing_edges: None,
    };

    vec![start_state,second_state,third_state,end_state,fail_state]
}


// Funzione per testare la proprietà rappresentata dall'automa su un flusso di operazioni
fn open_first(ops:OpFlow) -> bool {

    // Creo automa
    let op_first_auto : Vec<State> = get_open_first_automata();
    
    // Prelevo stato start
    let mut current_state = &op_first_auto[0];
    // Destruct OpFlow
    let OpFlow(mut op_seq) = ops;
    // Prelevo prima operazione richiesta    
    let mut curr_op = op_seq.pop();

    // Finchè non giungo ad uno stato finale
    while current_state.outgoing_edges.is_some() {

        // Controllo quale arco seguire
        for ed in current_state.outgoing_edges.as_ref().expect("REASON").iter() {

            // TO-DO: più di un arco true
            // Se la condizione di un arco è rispettata
            if (ed.condition)(curr_op.as_ref().expect("REASON").to_string()) {
                let mut next_state = current_state;
                let mut state_index = op_first_auto.iter();

                // Cerco prossimo stato nell'automa
                while next_state.name.ne(&ed.where_to) {
                    next_state = state_index.next().expect("REASON");
                }

                // Stato trovato
                if next_state.name.eq(&ed.where_to) {
                    current_state = next_state;
                    println!("{}",ed.where_to); // TEST
                }
                // Stato non presente
                else {return false};
            }
        }
        //Estraggo prossima operazione
        curr_op = op_seq.pop();
    }

    // Se finisco in uno stato di fallimento
    // -> proprietà non rispettata
    if current_state.name.eq("Fail") {
        return false;
    }
    // Altrimenti
    return true;
    
}

fn main() {
    
    // Controllo prima politica
    quickcheck(open_first as fn(OpFlow) -> bool);

}
