# Challenge 01 - rust_actixweb_token
first project Challenge for Rust Actix Web 

## Authentificated Token 
This is small program for try developed in rust your mission is complet the route /checktoken

### Prerequies
* capture the token available in the request
* define if is correct token or not
* check if token are expired
* the token is send to the header in request GET
* Return the expiration date for the token if valid.
  
you are other route for check credential and assign one expire date to token route is /login  
send just POST method username and password for check token and assign the expire date  
  
## validation
send me your pull request for look your result  
  
## Help
for install rust (rustup/cargo/rustc) packages use this cmd:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Run this application 
```
cd rust_actixweb_token
cargo run
```

curl for login initialized token if expired account
```
curl -X POST -d "username=titi&password=test" http://127.0.0.1:8080/login
```

curl for checktoken if token is available and not expired
```
curl -X GET -H "Authorization: YOUR_TOKEN" http://127.0.0.1:8080/checktoken
```
