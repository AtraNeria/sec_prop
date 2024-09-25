Part of a thesis exploring property based testing applied to security concerns.

The goal is to statically analyze the safety of a program's flow.
The safety properties are represented with automata, and are tested on randomized inputs using the quickcheck crate.

Example tested safety policies will include cases regarding access to file and the use of third-party API.
