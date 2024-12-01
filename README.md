# Advent of Code 2024
### Install
* Clone this repo
* Install Rust: https://www.rust-lang.org/tools/install
* Build the executable using `cargo build --release`
* Find the executable in `target/release/advent2024`

### Downloading data for AoC
* Login on [Advent of Code's website](https://adventofcode.com/2024)
* Obtain a session code identifying you to the AoC server. To do this, using Firefox:
    - In your browser, right click the page and press "inspect"
    - In the "Network" tab, press the "Reload" button
    - Click the HTML document
    - Under "Headers", in "Request headers", find your cookie.
    - Part of the cookie has the string `session=[long hexadecimal code];`. The hexadecimal part of this is your session key.
* Download days `x`, `y` and `z` into a directory `data` run: `advent2024 download [session key] data x y z`
* To download all released days, you can run `advent2024 [session key] download data --all`

Example:
```shell
$ advent2024 download data 9f5d642957086d6ab635fe1a1ccfdc2db09379dfcb9d8d0f07553fcc0528d9aae1355b1a84d384119823136e7aa411fc1412e950048a97efeca7d948d291c65d --all
```

### Solving days
* Make sure you've downloaded the data first e.g. into a directory called `data` (see the section above)
* To solve days `x`, `y` and `z`, run: `advent2024 solve data 1 2 3`
* Alternatively, to run all implemented days, run `advent2024 solve data --all`

Example:
```shell
$ advent2024 solve data 1 2 3
Day 01 [102.88µs]:
  Part 1: 421
  Part 2: 613

Day 02 [56.67µs]:
  Part 1: 10021
  Part 2: 2452123

Day 03 [278.12µs]:
  Part 1: 817
  Part 2: 22173

```

