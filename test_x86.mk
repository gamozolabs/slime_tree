all:
	cargo build --release --target=i686-linux-android
	adb reverse tcp:13370 tcp:13370
	adb push target/i686-linux-android/release/slime_tree /data/local/tmp/slime_tree
	adb shell chmod 755 /data/local/tmp/slime_tree
	adb shell /data/local/tmp/slime_tree

