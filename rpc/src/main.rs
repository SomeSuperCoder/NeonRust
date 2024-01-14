use rocket::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, get_neon_balance, get_token_balance])
}

#[get("/")]
fn hello() -> String {
    String::from("Neon RPC")
}

#[get("/balance/<address>")]
fn get_neon_balance(address: String) -> String {
    String::from("0")
}

#[get("/balance/<address>/<token>")]
fn get_token_balance(address: String, token: String) -> String {
    String::from("0")
}
