# A Rust CLI tool: password manager
This is a password manager CLI that allows users to store and retrieve passwords for different services. I use sled library, which provides a key-value store that can be used to store the encrypted passwords. 

## Project Setup
1. clone the repo:
```
git clone https://github.com/JuliaJHL/week10-rust-mini-proj.git
```
2. cd into the project:
```
cd week10-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run
```

## examples
* Every time you run the project, it will prompt you to choose a command.
* `store` prompts the user for a service name and password, encrypts the password, and stores the encrypted password in the database using the service name as the key.
* `retrieve` prompts the user for a service name and retrieves the corresponding encrypted password from the database. If a password is found, it is decrypted and printed to the console.
* `quit` will exit the project.
![week10](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/week10.png)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
