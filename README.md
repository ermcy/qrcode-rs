A small http server that generates a QR code for the given input.
For running locally, first clone the repo using
```shell
git clone http://github.com/realmercy/qrcode-rs
```
Then, run the following command to start the server
```shell
cargo build --release
cargo run
```
The server will be running on port 8080 with two endpoints

`http://localhost:8080` which just sends `Hello World!` as a response

`http://locahost:8080/qr/<input>` which generates a QR code for the given input and sends the resulting image as a base64 encoded string, or it sends an error message if the input was too long.
you can decode the string using any base64 decoder and use the image as you wish.
In my making of this it seemed like the maximum length of the input was either 2331 or 2335 characters, but I'm not sure if that's the case for everyone.

This is my first rust project, so any feedback is appreciated.