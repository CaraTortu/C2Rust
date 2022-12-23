# Rust Reverse Shell

This is a reverse shell that is undetectable by all antivirus (Tested on 23/12/2022). All traffic and output is end-to-end encrypted using AES256

Edit the main.rs file to include the ip and port you want to connect to. After this, compile using

```sh
cargo build -r
```

Your new compiled binary should be in client/target/release/client.exe

Now that you have your client generated. To set up and start the server run

```sh
cd server
python3 -m pip install pycryptodome
python3 ./main.py
```

Then you should get a reverse shell back! Enjoy :)
