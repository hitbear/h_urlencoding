# Hurlencoding

*URL encoding and decoding with Rust*

## Remark
This is a tool to URL encode and decode fast on the command line. It is fast and safe - as safe as Rust.

## Building and Installation

To compile the binaries run

    cargo build

or

    cargo build --release

The binaries `urldecode` and `urlencode` are in the `target/` directory.

    cp target/release/urlencode /usr/bin/

## Time and Efficency 

How fast is this program?

Use the shell script in the time_measurement directory to run a challange. 


### With sed alias
`$ alias urldecode_alias='sed "s@+@@g; s@%@\\\\x@g" | xargs -0 printf "%b"'`

`$ echo http%3A%2F%2Fwww%2Eexample%2Enet%2Findex%2Ehtml%3Fsession%3DA54C6FE2%23info | urldecode_alias 
http://www.example.net/index.html?session=A54C6FE2#info`

    $ time echo http%3A%2F%2Fwww%2Eexample%2Enet%2Findex%2Ehtml%3Fsession%3DA54C6FE2%23info | urldecode_alias 
    http://www.example.net/index.html?session=A54C6FE2#info

    real	0m0,007s
    user	0m0,004s
    sys	0m0,000s

### With Rust 
    $ time target/debug/hurlencoding http%3A%2F%2Fwww%2Eexample%2Enet%2Findex%2Ehtml%3Fsession%3DA54C6FE2%23info
    http://www.example.net/index.html?session=A54C6FE2#info

    real	0m0,002s
    user	0m0,001s
    sys	0m0,000s

## Development

### Testing
Run 

    cargo test

to run some tests defined in the `lib.rs` file. 

### Still to do

  * Encode and decode binary files.


## References
Thanks to providing the Rust crate urlencoding at
[https://github.com/kornelski/rust_urlencoding](URL)!