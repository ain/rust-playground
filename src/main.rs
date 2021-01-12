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
extern crate memory_ownership;
extern crate stringformatter;
extern crate traits;


use controlflow::status_match;
use collections::hashset_subset;
use stringformatter::formatted_string;
use datastructures::typed_matrix;
use datastructures::tuple_destruct;
use functions::odd_even_letters;
use traits::fishtank;
use memory_ownership::primitive_ownership;
use lifetime::ocean;
use reference_counter::strong_refs;
use mutex::refs_across_threads;
use circular_references::circ_refs;
use concurrency::spawner;
use external_dependencies::random_boolean;

fn main() {

    status_match();

    typed_matrix();
    tuple_destruct();

    hashset_subset();

    formatted_string();

    odd_even_letters();

    fishtank();

    primitive_ownership();

    ocean();

    strong_refs();
    refs_across_threads();
    circ_refs();

    spawner();

    random_boolean();
}
