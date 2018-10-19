all:
	cargo build --release --target=arm-linux-androideabi
	adb reverse tcp:13370 tcp:13370
	adb push target/arm-linux-androideabi/release/slime_tree /data/local/tmp/slime_tree
	adb shell chmod 755 /data/local/tmp/slime_tree
	adb shell /data/local/tmp/slime_tree

