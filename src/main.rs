extern crate mutex;
extern crate reference_counter;
extern crate circular_references;
extern crate collections;
extern crate concurrency;
extern crate controlflow;
extern crate datastructures;
extern crate external_dependencies;
extern crate functions;
extern crate lifetime;

use mutex::refs_across_threads;

use controlflow::status_match;
use collections::hashset_subset;
mod stringformatter;
use datastructures::typed_matrix;
use datastructures::tuple_destruct;
use functions::odd_even_letters;
mod traits;
mod memory_ownership;
use lifetime::ocean;
use reference_counter::strong_refs;
use circular_references::circ_refs;
use concurrency::spawner;
use external_dependencies::random_boolean;

fn main() {

    status_match();

    typed_matrix();
    tuple_destruct();

    hashset_subset();

    //stringformatter::formatted_string();

    odd_even_letters();

    //traits::fishtank();

    //memory_ownership::primitive_ownership();

    ocean();

    strong_refs();
    refs_across_threads();
    circ_refs();

    //concurrency::spawner();

    random_boolean();
}
