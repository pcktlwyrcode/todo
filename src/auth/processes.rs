use actix_web::dev::ServiceRequest;
/*
First, it takes in a reference of a service request.
It then passes that to a function that extracts the header, returning an error that's then returned in the function provided in step 1.
The extract header function then passes it to a check password function, which returns a result.
The result of the check password function is then returned to the initial function in step 1.

*/
use super::jwt;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(message) => Err(message)
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_password) => Ok(String::from(processed_password)),
                Err(_processed_password) => Err("there was an error processing token")
            }
        },
        None => Err("there is no token!")
    }
}

