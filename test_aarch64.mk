all:
	adb reverse tcp:13370 tcp:13370
	cargo build --release --target=aarch64-linux-android
	adb push target/aarch64-linux-android/release/slime_tree /data/local/tmp/slime_tree
	adb shell chmod 755 /data/local/tmp/slime_tree
	adb shell /data/local/tmp/slime_tree

