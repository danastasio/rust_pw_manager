# rust_pw_manager

This is an exceptionally shitty command line password manager. It's mostly just for me to hack on rust a bit. I don't really recommend using it.

## Usage

You will need an existing database, which you can initialize with the following:

```bash
touch password.db && echo "create table pw_store (key TEXT, password TEXT);" | sqlite3 password.db
```

Then just run the program 

```bash
pwstore
```

## Specificitcs

The program stores the passwords in the DB using AES encryption. The program starts by asking for a key from the user. This key is used to encrypt and decrypt passwords from the DB. It is not stored anywhere, and using the wrong key to retrieve the password simply results in a decryption failure.
