use std::collections::HashMap;

fn main()
{
    let mut hash_map = HashMap::new();
    hash_map.insert("Matematica", 90);
    hash_map.insert("Português", 72);
    hash_map.insert("Biologia", 58);
    hash_map.insert("Informática", 96);

    println!("O aluno cursou {} matérias", hash_map.len());

    match hash_map.get("Informática")
    {
        Some(k) => println!("Informática => nota: {}", k),
        None => println!("Não cursou informática")
    }

    hash_map.remove("Português");
    println!("Estuda português? {}", hash_map.contains_key("Português"));
    println!("Estuda biologia? {}", hash_map.contains_key("Biologia"));
}
