mod controlflow;
mod collections;
mod stringformatter;
mod datastructures;
mod functions;
mod traits;
mod memory_ownership;
mod lifetime;
mod reference_counter;
mod mutex;
mod circular_references;
mod concurrency;

fn main() {

    //controlflow::status_match();

    //datastructures::typed_matrix();
    //datastructures::tuple_destruct();

    //collections::hashset_subset();

    //stringformatter::formatted_string();

    //functions::odd_even_letters();

    //traits::fishtank();

    //memory_ownership::primitive_ownership();

    //lifetime::ocean();

    //reference_counter::strong_refs();
    //mutex::refs_across_threads();
    //circular_references::circ_refs();

    concurrency::spawner();
}
