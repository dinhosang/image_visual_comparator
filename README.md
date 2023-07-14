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
  - most would be private so not generated in cargo doc

TODO:

- how much should this cli do ?
  - take in dirs of image to sweep through
    - one set for original, and one for recently taken
  - grab pairs of images and do comparison
    - images need same name but be in different folders
    - how many to grab at once ? multithreading ? would need to benchmark to know
  - take each resulting list of mismatched pixels
  - create new image that is the pair of images side by side
  - draw box around areas of mismatched pixels on both sides of this new image
    - box should be close enough to the pixels to be clear what is wrong
    - box should be able to enlargen and encompass multiple mismatched pixels if they are close enough to one another
    - box should default red, but could be another colour ?
      - set by user ?
      - determined by colours on image ?
  - save new image into a results folder
  - report back to user what the result was ?
  - should log progress
    - can differ in style if logging to terminal or to file ?
    - should allow or logs to be suppressed
  - should write result to a file in top of results folder so that other apps can see progress and respond as appropriate ?
- should not handle:
  - responding to there being mismatched images, caller of cli should see result file and decide what they want to do
  - terminal interaction by user
    - should just take starting input and run
- figure out how I want to get input from user and what shape it should take
  - args and/or config file and/or env keys ?
    - <https://crates.io/crates/config>
- allow passing in of tolerance (i.e. what % diff will allow)
- figure out how benchmarking works
  - checking memory usage too ? and other metrics
- create many images
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
- io_uring for io operations ?
