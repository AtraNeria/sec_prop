use crate::code_provider_structs::{CodeProvider, ApiFunCall};


// Struct representing outgoing edges from a state of the automata
pub struct Edge {

    // Condition to follow this edge
    pub condition: Box<dyn Fn(String)->bool>,
    // Name of the state to which the edge goes
    pub where_to: String,

}

// Struct representing a state of the automata
pub struct State {
    
    // Name of the state to be used by edges
    pub name: String,
    // Action to be performed in the state
    pub action: Option<Box<dyn Fn(CodeProvider,ApiFunCall)-> String>>,

    // Flags for what kind of state it is
    pub is_starting: bool,
    pub is_terminal: bool,
    pub is_failure: bool,
    
    // Edges from this state
    pub outgoing_edges: Option<Vec<Edge>>,

}

// Struttura che contiene il risultato di un'istanza di test
pub struct Result {
    // Codice del risultato
    pub result_code: u8,
    // Numero di stati esplorati
    pub explored_states: u8,
    // Possibile prossimo stato
    // usato se lo stato non viene trovato
    pub next_state_unfound: Option<String>,
    // Edge resi true dall'ultima operazione
    // se più edge sono stati resi true
    pub true_edges: Option<Vec<EdgeId>>,
}

// Struttura per identificare un arco
pub struct EdgeId {
    pub from_state: String,
    pub to_state: String,
}
