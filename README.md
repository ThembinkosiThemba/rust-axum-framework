# AXUM API

Axum is a web application framework that focuses on ergonomics and modularity for Rust.
This project was made to try out Axum in Rust to make a functional restful api.

The API lets you do CRUD actions on Posts and Reviews.
The API contains a generic controller, connection to mongoDB, authentication via jsonwebtokens and encryption for passwords.

## Goals for this project

- Connect to a database
- CRUD requests
- several models
- encrypt passwords of users
- use jsonwebtokens for requests
- middleware
- a model with complex datatypes (time, lists e.g.)
- only objects can be deleted/edited by the author

Instructions -

1. replace your mongodb string in the mongo.rs file
2. run the program with cargo run

create user -
POST localhost:8000/users/register

{
"username":"bruce",
"password":"wayne"
}

login user -
GET localhost:8000/users/login
{
"username":"bruce",
"password":"wayne"
}
