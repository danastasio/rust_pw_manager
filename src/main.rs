#[macro_use] extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;
use rusqlite::{Connection, Result, params};
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

struct Pw {
    encrypted_password: String,
}

fn main() {
    let clear = || { print!("{}[2J", 27 as char); };
    let mut user_key: String = String::new();
    print!("Enter the password store key: ");
    stdout().flush().expect("ERROR");
    stdin().read_line(&mut user_key).expect("ERROR");
    clear();
    loop {
        let mut user_choice: String = String::new();
        stdout().flush().expect("ERROR");
        print!("Welcome to my password storage.
Please choose an option:

1) Store a password
2) Retrieve a password
3) Quit/Exit
> ");
        stdout().flush().expect("ERROR");
        stdin().read_line(&mut user_choice).expect("ERROR");
        let num: u64 = match user_choice.trim() {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            _   => 9999,
        };
        match num {
            9999 => { println!("Invalid choice. Try again"); },
            1 => { 
                clear(); 
                let mut pw_key: String = String::new();
                let mut raw_pw: String = String::new();
                println!("Enter the name of the website");
                stdin().read_line(&mut pw_key).expect("ERROR");
                stdout().flush().expect("ERROR");
                clear();
                println!("Enter the password to store");
                stdin().read_line(&mut raw_pw).expect("ERROR");
                stdout().flush().expect("ERROR");
                let encrypted_pw: String = encrypt_pw(String::from(raw_pw.trim()), String::from(user_key.trim()));
                write_to_db(pw_key, encrypted_pw).expect("There was a problem during write");
            },
            2 => {
                clear();
                let mut pw_key: String = String::new();
                println!("Enter the password key: ");
                stdin().read_line(&mut pw_key).expect("ERROR");
                stdout().flush().expect("ERROR");
                let encrypted_pw = match get_from_db(String::from(pw_key.trim())) {
                    Ok(n) => n,
                    Err(..) => String::from("errordb01")
                };
                if encrypted_pw == "" {
                    println!("{}", "Error getting password from DB. Maybe not found?\n\n");
                } else {
                    let password: String = decrypt_pw(encrypted_pw, String::from(user_key.trim()));
                    println!("The password is: {}\n\n", password);
                }
            },
            3 => { break; },
            _ => { println!("Invalid, invalid choice"); },
        }
    }

}

fn encrypt_pw(unencrypted: String, user_key: String) -> String {
    let mc= new_magic_crypt!(user_key, 256);
    let base64 = mc.encrypt_str_to_base64(unencrypted);
    return base64;
}

fn decrypt_pw(encrypted: String, user_key: String) -> String {
    let mc = new_magic_crypt!(user_key, 256);
    let decrypted = match mc.decrypt_base64_to_string(encrypted) {
        Ok(ref n) => String::from(n),
        Err(..) => String::from("There was an error decrypting the password. Possibly using the wrong key?"),
    };
    return decrypted;
}

fn write_to_db(pw_key: String, encrypted_pw: String) -> Result<()> {
    let conn = Connection::open("passwords.db")?;
    conn.execute("INSERT INTO pw_store ('key','password') values (?1, ?2)", params![pw_key.trim(), encrypted_pw])?;
    Ok(())
}

fn get_from_db(pw_key: String) -> Result<String> {
    let conn = Connection::open("passwords.db").expect("Could not open DB");
    let mut stmt = conn.prepare("SELECT key,password from pw_store where key=:key;").expect("Could not prepare query");
    let result_iter = stmt.query_map(&[(":key", pw_key.to_string().as_str())], |row| {
        Ok(Pw { encrypted_password: row.get(1)? })
    })?;
    let mut encrypted_pw = String::new();

    for pw in result_iter {
        let pass = pw?;
        encrypted_pw = pass.encrypted_password;
    }

    Ok(encrypted_pw)
}
