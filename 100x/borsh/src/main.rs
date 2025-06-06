use borsh::{to_vec, BorshDeserialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Debug,BorshDeserialize,BorshSerialize)]
struct User {
    username : String,
    password : String
}

fn main() {
    let user = User {
        username : String::from("krishnajayesh"),
        password : String::from("howareyou")
    };

    let bytes = to_vec(&user).unwrap();
    let ctuser = User::try_from_slice(&bytes).unwrap();
    println!("{:?}", bytes);
    println!("{:?}", ctuser);

}