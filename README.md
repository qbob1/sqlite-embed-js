# sqlite-embed-js
quick js embedded in sqlite

#build
cargo b

#load into sqlite
.load ./target/debug/libembedjs.dylib

#sqlite
select js("'hello world'")
