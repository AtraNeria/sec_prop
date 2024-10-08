use quickcheck::quickcheck;
mod automata_structs;

use file_testing::{open_first, read_only, single_write};
mod file_testing;

use operations_structs::OpFlow as OpFlow;
mod operations_structs;


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