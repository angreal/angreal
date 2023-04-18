---
title: Contributing
weight: 50
---

Angreal is hosted on [github](https://github.com/angreal/angreal).

If you have questions, concerns, bug reports, or suggestions please feel
free to open an [issue](https://github.com/angreal/angreal/issues),
I'll do my best to get it addressed.

If you'd like to contribute back to angreal's code base (or
documentation!) feel free to submit a [pull
request](https://github.com/angreal/angreal/pulls).

Before submitting a merge request, it would be best if you open a new
issue that outlines what the problem you wish to solve is (and perhaps
see if anyone else is working on it).

## Development setup

Angreal uses angreal! Tasks can be confidently run on :
- linux
- Windows WSL
- MacOSX


1. `pip install angreal`, install angreal for executing our defined tasks
1. `angreal bootstrap-dev`, setup your environment
1. `angreal run-tests`, run our test suite


### Windows Development Support

I did not have windows in mind when writing initial angreal tasks, they may (probably) won't work
well out of the box.

1. When running `maturin develop` you may see failures in the build chain due to OpenSSL not being installed. Installing [StrawberryPerl](https://strawberryperl.com/)
SHOULD take care of that for you.

2. Tests can be run via `cargo test` and `pytest`.
