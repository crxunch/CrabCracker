# CrabCracker

## Description

CrabCracker is a bare-bones password cracking tool written in Rust.

CrabCracker is currently in the very first stage of production and while it can crack password hashes, it is limited by the following:

1. It only supports NTLM hashing.
2. It does not support multi-threading.

CrabCracker is soon to have the following improvements:

1. Support for multiple hashing algorithms.
2. Multi-threading.
3. Hashlist parsing:
    1. Organize hashlist by frequency (most often occurring hashes move to front of list).
    2. Remove duplicate hashes.
    3. Automatically parse `/etc/shadow` files.
4. Support for output files.

## Usage

To run CrabCracker, clone the repository, then:

1. `cd /path/to/CrabCracker/`
2. `cargo run -- -f <hashlist> -w <wordlist> -a <hashing_algorithm>`

Once you have run CrabCracker for the first time, you may continue to run CrabCracker with the above command, or you may:

1. Navigate to `/path/to/CrabCracker/target/debug`
2. Run the `CrabCracker` binary as you would any other.
