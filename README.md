# Some things I'd like to try in this repo

- rust
- image comparison
- concurrency / parallelism if appropriate

If wishing to PR:

- ensure you've installed cargo-make:
  - cargo install cargo-make
  - run the install task
    - cargo make install

TODO NEXT:

- communicating back result of spawned jobs

TODO NEXT NEXT:

- add tests for lib.rs

TODO:

- development container
- production container
- refactor directory structure of utils
- take each resulting list of mismatched pixels
- create new image that is the pair of images side by side
- draw box around areas of mismatched pixels on both sides of this new image
  - box should be close enough to the pixels to be clear what is wrong
  - box should be able to enlargen and encompass multiple mismatched pixels if they are close enough to one another
  - box should default red, but could be another colour ?
    - set by user ?
    - determined by colours on image ?
- refactor test_utils
- fix and refactor bench tests to create the files for the test, also to create actual config, then turn on again in pre commit hooks
- possible for benchmark results to be somewhere that can be stored for running in CD environment ?
- can test main.rs ?
- test AppConfig
- any vecs that can be converted to just being slices ?
- save new image into a results folder
- report back to user what the result was ?
- should write result to a file in top of results folder so that other apps can see progress and respond as appropriate ?
- should not handle:
  - responding to there being mismatched images, caller of cli should see result file and decide what they want to do
  - terminal interaction by user
    - should just take starting input and run
- figure out how benchmarking works
  - checking memory usage too ? and other metrics
- stress test for benchmark
  - do all sync first on one thread
  - then benchmark with stress testing
  - then try rust conc / parallel
  - see which improves for benchmarking
  - also see if there are diminishing returns when there are fewer images
    - would maybe swap between single-thread/sync and the opposite depending on image number or number of cores, or amount of memory being allocated to cli ?
      - sysinfo crate can see memory (including allocated?) and other system info
- profiling as well as benchmarking
  - looks like there are several rust crates for profiling
- actual make rest of tool
- perhaps add functionality to fail quickly
  - don't bother finding all mismatched pixels, if you find one then just break out and report a mismatch
- look at error handling again - consider thiserror crate possibly ?
- figure out package lock business
- how does coverage work ?
- logging using tracing ?
- proptesting ?
- fuzz testing ?
- mutagen testing ?
- static analysis ?
- async logging ?
- some kind of loading bar , update, message so folks know how many have been done or % of progress?
  - might not make sense depending on what the end result of this repo is
- for benchmarking, figure out how to store the criterion report created on each PR and have new runs compare against the last one.
  - FIRST: is it correct to use Criterion in CI ? Docs for crate so no :/ so maybe just do locally, if so can ignore all below
  - is it possible to store the report somehow in github but outside of the .git contents itself ?
  - check for github actions
  - can compare previous report and new for regression and fail if so
  - if no regression then can upload new report as baseline for future PRs ?
  - could create separate github workflow just for benchmarking
    - need to remove from regular workflow then
      - possible help:
        - <https://github.com/marketplace/actions/criterion-compare-prs>
        - <https://blog.petitviolet.net/post/2020-10-08/github-action-for-rust-project>
        - actions/download-artifact@v2
        - actions/upload-artifact@v2
- benchmarking
  - docs for Criterion mention it's not suitable for CI like GitHub Actions, and shuold use Iai instead :/
    - <https://bheisler.github.io/criterion.rs/book/faq.html>
    - <https://bheisler.github.io/criterion.rs/book/iai/comparison.html>
  - extension to criterion (no idea if useful to me):
    - <https://crates.io/crates/criterion-perf-events>
    - <https://crates.io/crates/criterion-cycles-per-byte>
  - maybe better to enforce running locally and check for regression locally rather than on CI ?
- check through this when done to check good practices are followed:
  - <https://rust-cli.github.io/book/index.html>
- making cross platform
  - does the fs module handle taking unix style paths and making them work in a windows os ? or does the cli have to do that itself/via a crate ?
- update pr checks yaml to lock down version of rust used as clippy rules can change from version to version
- look at any other TODO
- not for this, but in <https://github.com/dinhosang/steam_aws/blob/main/docs/how_to_use.md#remote-desktop-access> , clean up readme - has some grammar issues. Also check and write explicitly about what I do regarding ip addresses/security of the of ec2 instance. I'm pretty sure I do lock that down but I should say that explicitly i I intend this to function as a showpiece of things I think about.
