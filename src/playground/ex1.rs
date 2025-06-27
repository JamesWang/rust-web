pub fn ex1_func() {
    let mut address = String::from("Street 1");
    add_postal_code(&mut address);

    println!("{}", address);
}

fn add_postal_code(address: &mut String) {
    address.push_str(", 12345 Kingston");
}