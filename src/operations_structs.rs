use quickcheck::{Arbitrary,Gen};
use std::fmt;

// Vettore che contiene il set di operazioni valide
const POSSIBLE_OPS: &[&str] = &["CR","OP","WR","RD","CL"];

// Struttura wrapper per la fila di operazioni da eseguire
#[derive(Clone, Debug)]
pub struct OpFlow (Vec<String>);

// Implementazione di Arbitrary per generare una lista random di operazioni
impl Arbitrary for OpFlow {
    fn arbitrary(g: &mut Gen) -> OpFlow {
        let mut generated: Vec<String> = vec![];
        for _i in 0..=5 {
            let genops =  usize::arbitrary(g) % 5;
            generated.push(POSSIBLE_OPS[genops].to_string());
        }
        //Return
        OpFlow(generated)
    }
}

// Implemento Display per testing
impl fmt::Display for OpFlow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}
