Changelog
=========

1.0.2
-----

* Updated dependencies
* Build with Rust 1.28.0

1.0.1
-----

* Updated dependencies
* Build with Rust 1.27.0
* Fix: Replaced `io::Write::write` with `io::Write::write_all`. The former may not write all the
       bytes, which can result in invalid output. The symptom could not be produced by a test, but
       has been indicated by a linter (clippy).

1.0.0
-----

* MIT License
* Support for stdin and stdout
* Support for customer extensions

0.3.x
-----

introduced to test release process

0.2.3
-----

`--category` is now a positional argument.

0.2.2
-----

New `--record-type` option.

0.2.1
-----

New `--delimiter` option.