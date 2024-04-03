mod ctci_stacks_3_2;
mod ctci_stacks_3_4;
mod stack;
mod graph;
mod ctci_graphs_4;

fn main() {
    let s = "3_2";

    match s {
        "3_2" => ctci_stacks_3_2::run(),
        "3_4" => ctci_stacks_3_4::run(),
        _ => println!("No exercise selected")
    }
    
    
}
