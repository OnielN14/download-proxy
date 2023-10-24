# Download Proxy

Download Proxy is a proxy server to download file.

## Developing Locally
Make sure you have [`rustup`](https://rustup.rs/) and cargo installed. Then run the following command:
```bash
cargo build -r -p api
```

## Usage
The API will use this address `127.0.0.1:8080`, but we may change the host and port separately by passing `--host` and `--port` when running the binary.

Run the following command to print API execution option.
```bash
api -h
```
```bash
Usage: api [OPTIONS]

Options:
      --host <HOST>  [default: 127.0.0.1]
  -p, --port <PORT>  [default: 8080]
  -h, --help         Print help information
  -V, --version      Print version information
```

## Run From Docker
```bash
docker pull ghcr.io/onieln14/download-proxy:latest
```
|Environment Variable|Description|
|--------------------|-----------|
|HOST|Specify host value. Default `0.0.0.0`|
|PORT|Specify port value. Default `8080`|


## Endpoint

|Method|Endpoint|Description|
|------|--------|-----------|
|GET|/download?url=\<url\>&filename=\<filename\>|Download a file from URL provided by `url` param. The downloaded file can be renamed by passing a value to `filename` param|
|POST|/download|The endpoint accept json body or form that consists of `url` and `filename`|

### Example
|Endpoint|Description|
|--------|-----------|
|/download?url=https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf|Download PDF file from https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf|
|/download?url=https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf&filename=non-dummy.pdf|Download PDF file from https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf and rename the file into `non-dummy.pdf`|

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT License](LICENSE)