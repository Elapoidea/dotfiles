mod path_manager;

fn main() {
    let arg = std::env::args().nth(1).expect("arg not found");
    let value = match std::env::args().nth(2) {
        Some(n) => n,
        None => "".to_string()
    };

    let palletes = path_manager::Themes::new();

    match arg.as_str() {
        "add" => palletes.add_path(value),
        "del" => palletes.del_path(string_to_usize(value) - 1),
        "use" => palletes.use_theme(string_to_usize(value) - 1),
        "apply" => palletes.apply_to_apps(),
        "list" => palletes.search_paths(value),
        _ => println!("invalid arg")
    }
}

fn string_to_usize(value: String) -> usize {
    value.as_str().parse::<usize>().expect("failed to parse string")
}
