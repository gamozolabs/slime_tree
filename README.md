# slime_tree

The worlds worst Android fuzzer.

Modify `.cargo/config` to point to your unzipped `ndk`.

To fuzz run `make -f test_aarch64.mk` or `make -f test_arm.mk`.

The makefile should work on Windows with `nmake` as well, you might just need to fix up some paths and convert `/` to `\\`

You also can run the server by going into `slime_tree_server` and running `cargo run`

