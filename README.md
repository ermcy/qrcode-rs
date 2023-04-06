A small http server that generates a QR code for the given input.
For running locally, first clone the repo using
```shell
git clone http://github.com/realmercy/qrcode-rs
```
Then, run the following command to start the server
```shell
cargo build --release
```
The server will be running on port 8080 with two endpoints

./ which just sends `Hello World!` as a response

./qr/<input> which generates a QR code for the given input and sends the resulting image as a base64 encoded string
you can decode the string using any base64 decoder and use the image as you wish.

This is my first rust project, so any feedback is appreciated.