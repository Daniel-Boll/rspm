use crate::file::get_passwords;

/// List all passwords name registered
pub fn list_passwords() {
    let passwords = get_passwords();

    passwords["passwords"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|password| {
            println!("{}", password["name"].as_str().unwrap());
        });
}
