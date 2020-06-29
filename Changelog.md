Changelog
=========

2.0.3
-----

* Updated dependencies, build with Rust 1.44.

2.0.2
-----

* Patched upstream version of `quick-xml`, to fix an issue with bytes dropped from compressed output.

2.0.1
-----

* Criterion Benchmark Suite

2.0.0
-----

* Breaking change: --category and --input options are now mandatory.
* Small speedup than using stdin, stdout.

1.0.17
------

* Update dependencies
* Use flate2 for compression / decompression
* Speedup for compressing / decompressing files, through inserting io buffers.

1.0.16
------

* Support for zipped file extensions

1.0.15
------

* Updated dependencies
* Build with Rust 1.40.0
* Does not show progress bar if writing to stdout

1.0.14
------

* Updated dependencies
* Build with Rust 1.39.0

1.0.13
------

* Updated dependencies
* Build with Rust 1.38.0

1.0.12
------

* Updated dependencies
* Build with Rust 1.37.0

1.0.11
------

* Updated dependencies
* Build with Rust 1.36.0

1.0.10
------

* Visual C Runtime is now linked statically into windows executable and no longer a runtime requirement.

1.0.9
-----

* Updated dependencies
* Build with Rust 1.35.0

1.0.8
-----

* Update dependencies
* Build with Rust 1.34.0

1.0.7
-----

* Updated dependencies
* Build with Rust 1.33.0

1.0.6
-----

* Fixed a Bug during escaping character data for XML. It did cause a panic or faulty output, if in
  the sequence to escape character points larger than one byte did appear before the first escaped
  character.

1.0.5
-----

* Updated dependencies
* Build with Rust 1.32.0

1.0.4
-----

* Updated dependencies
* Build with Rust 1.30.0
* Enabled link time optimizations

1.0.3
-----

* Updated dependencies
* Build with Rust 1.29.0

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
