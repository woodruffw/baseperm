baseperm
========

A small tool for generating valid permutations of strings in base*N* alphabets.

## Theory of Operation

Many popular binary-to-printable serialization/encoding schemes use
alphabets whose bitnesses do not allow 8-bit bytes to fit evenly inside a symbol
(or multiple symbols):

* [base32](https://en.wikipedia.org/wiki/Base32): 5 bits
* [base64](https://en.wikipedia.org/wiki/Base64): 6 bits
* [base58](https://en.wikipedia.org/wiki/Base58): \~5.86 bits

Consequently, these encodings employ padding schemes to round their outputs to 8-bit multiples.

`baseperm` manipulates the padding bits in these encodings to produce distinct, valid encoded
forms that decode to the same input.

## Why?

Programmers frequently make the mistake of assuming that encoded representations have a 1-1
correspondence with their inputs. This results in all kinds of interesting, potentially exploitable
errors:

* Ratelimiting bypasses due to keying on the serialized form

* Dedeuplication and reuse bypasses

* Forced dictionary collisions

## Installation

`baseperm` is a single command-line program. You can install it using `cargo`:

```bash
cargo install nvis
```

Or by building it locally:

```bash
git clone https://github.com/woodruffw/baseperm && cd baseperm
cargo build
```

## Usage

`baseperm` takes a permutation candidate on `stdin` and writes all permuted equivalent forms
to `stdout`, separated by newlines. The original input is also included in the output, and (RFC4648)
base64 is the default.

```bash
echo "hello!" | base64 | baseperm
```

Alternative encodings can be specified with `-e`, `--encoding`:

```bash
echo "hello!" | base32 | baseperm -e base32
```

See `baseperm -h` for a full list of supported encodings.
