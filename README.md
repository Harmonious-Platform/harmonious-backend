# Harmonious Backend
## How to host
Download the source code and set `DATABASE_URL` `SECRET_KEY` `SENDING_EMAIL_ADDRESS` `SMTP_PASSWORD` `SMTP_SERVER` `SMTP_USERNAME` `RUST_BACKTRACE=full` in .env in the main harmonious folder.  
The server will be hosted on localhost:3000 unless it's not locally hosted and DOMAIN is set in the .env, to set the port set the PORT variable in the .env file.  
[Install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [postgres](https://www.postgresql.org/download/)  
Run `cargo install diesel_cli --no-default-features --features postgres` `diesel setup` `diesel migration generate users` `diesel migration generate invitations` in the harmonious folder.  
Then simply run `cargo run`
