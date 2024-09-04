# Multithreaded HTTP Server in Rust

This is a simple multithreaded HTTP server written in Rust. It handles multiple client requests concurrently using threads. The server listens on `127.0.0.1:6969` and responds with a basic "Hello, World!" message to any incoming HTTP request.

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Ensure that Rust is installed on your system)

### Clone the Repository

If you haven't already, clone the repository:

```
git clone https://github.com/hackice20/multi-threaded-server
cd multi-threaded-server
```
Build the Project

To build the project, run the following command:

```

cargo build
```
This will compile the project and generate an executable in the target/debug directory.
Run the Project

To run the server, use the following command:


```
cargo run
```
The server will start listening on 127.0.0.1:6969.
Testing the Server

You can test the server by navigating to http://127.0.0.1:6969 in your web browser. You should see a "Hello, World!" message.
Checking if the Server is Multithreaded

To verify that the server is handling requests in a multithreaded manner:

  Send Multiple Requests Simultaneously:
  Open multiple tabs in your web browser pointing to http://127.0.0.1:6969.
  Use a command-line tool like curl to send multiple requests simultaneously:

  
```
curl http://127.0.0.1:6969 &
curl http://127.0.0.1:6969 &
curl http://127.0.0.1:6969 &
```
Use a load-testing tool like ApacheBench (ab):


```
    ab -n 10 -c 5 http://127.0.0.1:6969/
```
Observe the Terminal Output:

   The server will print the thread IDs handling each request.
    If different thread IDs are shown for different requests, it confirms that the server is handling requests in a multithreaded manner.
    ![OutPut](https://github.com/hackice20/multi-threaded-server/blob/master/Screenshot%202024-09-04%20060752.png)
