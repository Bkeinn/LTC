# LTC
A lossy text compression algorithem

# Try it out
Currently I would only decode Text that does not included non ASCII characters.
## build
```
cargo build
```
## Encode
```
./target/release/ltc e --input normal.txt --output encoded.hm --type 0
```
## Decode
```
./target/release/ltc d --input encoded.hm --output decoded.txt --type 0
```
