# clasher

## Description

clasher (or, the Command Line hASH crackER) is a bare-bones password cracking tool written in Rust.

clasher is currently in the first stage of production, and while it can crack password hashes, it is limited by the fact that it does not yet support multi-threading.

clasher is soon to have the following improvements:

1. Multi-threading.
2. Hashlist parsing:
    1. Organize hashlist by frequency (most often occurring hashes move to front of list).
    2. Remove duplicate hashes.
    3. Automatically parse `/etc/shadow` files.
3. Support for output files.

## Usage

To run clasher, clone the repository, then:

1. `cd /path/to/clasher/`
2. `cargo run -- -f <hashlist> -w <wordlist> -a <hashing_algorithm>`

Once you have run clasher for the first time, you may continue to run clasher with the above command, or you may:

1. Navigate to `/path/to/clasher/target/debug`
2. Run the `clasher` binary as you would any other.
