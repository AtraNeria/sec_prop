use quickcheck::quickcheck;
mod automata_structs;

use file_testing::{open_first, read_only, single_write};
mod file_testing;
use API_testing::{conflict_test, isolation_test};
mod API_testing;

use operations_structs::OpFlow as OpFlow;
mod operations_structs;
use code_provider_structs::{CodeProvider,ApiFunCall};
mod code_provider_structs;

// Controllo prima politica
// Prima di poter eseguire un'operazione su un file questo deve essere aperto
#[test]
fn test_first () {
    quickcheck(open_first as fn(OpFlow) -> bool);
}


// Controllo seconda politica
// File read-only
#[test]
fn test_second () {
    quickcheck(read_only as fn(OpFlow) -> bool);
}


// Controllo terza politica
// Singola modifica in write permessa
#[test]
fn test_third () {
    quickcheck(single_write as fn(OpFlow) -> bool);
}

// Controlla prima politica su API
// Estraggo provider di codice e funzione chiamata, e controllo i conflitti
#[test]
fn test_api_first () {
    quickcheck(conflict_test as fn(CodeProvider, ApiFunCall) -> bool);
}

// Controlla seconda politica su API
// Controllo che si chiamino solo funzioni dello stesso provider
#[test]
fn test_api_second () {
    quickcheck(isolation_test as fn(CodeProvider, ApiFunCall) -> bool);
}
