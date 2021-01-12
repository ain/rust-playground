extern crate mutex;
extern crate reference_counter;
extern crate circular_references;
extern crate collections;
extern crate concurrency;
extern crate controlflow;
extern crate datastructures;

use mutex::refs_across_threads;

use controlflow::status_match;
use collections::hashset_subset;
mod stringformatter;
use datastructures::typed_matrix;
use datastructures::tuple_destruct;
mod functions;
mod traits;
mod memory_ownership;
mod lifetime;
use reference_counter::strong_refs;
use circular_references::circ_refs;
use concurrency::spawner;
mod external_dependencies;

fn main() {

    status_match();

    typed_matrix();
    tuple_destruct();

    hashset_subset();

    //stringformatter::formatted_string();

    //functions::odd_even_letters();

    //traits::fishtank();

    //memory_ownership::primitive_ownership();

    //lifetime::ocean();

    strong_refs();
    refs_across_threads();
    circ_refs();

    //concurrency::spawner();

    //external_dependencies::random_boolean();
}
