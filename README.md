# Challenge 01 - rust_actixweb_token
first project Challenge for Rust Actix Web 

## Authentificated Token 
This is small program for try developed in rust your mission is complet the route /checktoken
* cath the token available in the request
* define if is correct token or not
* check if token are expired

send me your pull request for look your result

you are other route for check credential and assign one expire date to token route is /login  
send just POST method username and password for check token and assign the expire date  
the token is send to the header in request GET.

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

curl for login -> /login
```
curl -X POST -d "username=titi&password=test" http://127.0.0.1:8080/login
```

curl for your route -> /checktoken
```
curl -X GET -H "Authorization: YOUR_TOKEN" -d "username=titi&password=test" http://127.0.0.1:8080/checktoken
```
