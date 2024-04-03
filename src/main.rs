mod ctci_stacks_3_2;
mod ctci_stacks_3_4;
mod ctci_graphs_4;
mod utils;

fn main() {
    let s = "4_1";

    match s {
        "3_2" => ctci_stacks_3_2::run(),
        "3_4" => ctci_stacks_3_4::run(),
        "4_1" => ctci_graphs_4::ctci_graphs_4_1::run(),
        _ => println!("No exercise selected")
    }
    
    
}
