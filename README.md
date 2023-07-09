Some things I'd like to try in this repo:

- rust
- image comparison
- concurrency / parallelism if appropriate

If wishing to PR:

- ensure you've installed cargo-make:
  - cargo install cargo-make
  - run the install task
    - cargo make install

TODO NEXT:

- add in unit testing
- documentation
  - what types are possible ?
    - gen doc, docs with examples, docs with tests ?

TODO:

- figure out how to get input from user and what shape it should take
  - args and/or config file ?
- finish function for comparing two images and return collection of non-matching pixels
- allow passing in of tolerance (i.e. what % diff will allow)
- figure out how benchmarking works
  - checking memory usage too ? and other metrics
- create many images
- stress test for benchmark
- try rust conc / parallel and see which improves for benchmarking
- actual make rest of tool
- perhaps add functionality to fail quickly
  - don't bother finding all mismatched pixels, if you find one then just break out and report a mismatch
- look at error handling again - consider thiserror crate possibly ?
- figure out package lock business
- how does coverage work ?
