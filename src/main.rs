mod controlflow;
mod collections;
mod stringformatter;
mod datastructures;
mod functions;

fn main() {

    controlflow::status_match();

    datastructures::typed_matrix();
    datastructures::tuple_destruct();

    collections::hashset_subset();

    stringformatter::formatted_string();

    functions::odd_even_letters();
}
