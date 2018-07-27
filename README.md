WAIT-FOR-RUST
=============

A very simple implementation of the `wait-for-it.sh` script in rust.

## Usage
This program will attempt to initiate a TCP connection to `<host:port>` for
30s. Once connected, it will either exit with status 0 or run the provided
command.

```
wait-for-rust <host:port> [command]
wait-for-rust <host:port> -- long command
```
