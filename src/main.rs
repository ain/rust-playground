extern crate mutex;
extern crate reference_counter;
extern crate circular_references;
extern crate collections;
extern crate concurrency;

use mutex::refs_across_threads;

mod controlflow;
use collections::hashset_subset;
mod stringformatter;
mod datastructures;
mod functions;
mod traits;
mod memory_ownership;
mod lifetime;
use reference_counter::strong_refs;
use circular_references::circ_refs;
use concurrency::spawner;
mod external_dependencies;

fn main() {

    //controlflow::status_match();

    //datastructures::typed_matrix();
    //datastructures::tuple_destruct();

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
