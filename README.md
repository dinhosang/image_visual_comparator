Some things I'd like to try in this repo:

- rust
- image comparison
- concurrency / parallelism if appropriate

If wishing to PR:

- ensure you've installed cargo-make:
  - cargo install cargo-make
  - run the install task
    - cargo make install

TODO:

- finish function for comparing two images and return collection of non-matching pixels
- allow passing in of tolerance (i.e. what % diff will allow)
- figure out how benchmarking works
- do what unit testing we can
- create many images
- stress test for benchmark
- try rust conc / parallel and see which improves for benchmarking
- actual make rest of tool
- perhaps add functionality to fail quickly
  - don't bother finding all mismatched pixels, if you find one then just break out and report a mismatch
