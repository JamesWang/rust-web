pub fn ex1_func() {
    let address = String::from("Street 1");
    add_postal_code(&mut address);

    println!("{}", a);
}

fn add_postal_code(mut address: String) {
    address.push_str(", 12345 Kingston");
}