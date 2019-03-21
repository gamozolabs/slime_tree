all:
	cargo build --target x86_64-linux-android --release
	adb reverse tcp:13370 tcp:13370
	adb push target/x86_64-linux-android/release/slime_tree /data/local/tmp/slime_tree
	adb shell chmod 755 /data/local/tmp/slime_tree
	adb shell /data/local/tmp/slime_tree

