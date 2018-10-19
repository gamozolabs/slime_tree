extern crate rand;

use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::net::TcpStream;

/// Maximum number of threads to fuzz with
const MAX_THREADS: u32 = 32;

/// Recursively list all files starting at the path specified by `dir`, saving
/// all files to `output_list`
fn listdirs(dir: &Path, output_list: &mut Vec<(PathBuf, bool, bool)>) {
    // List the directory
    let list = std::fs::read_dir(dir);

    if let Ok(list) = list {
        // Go through each entry in the directory, if we were able to list the
        // directory safely
        for entry in list {
            if let Ok(entry) = entry {
                // Get the path representing the directory entry
                let path = entry.path();

                // Get the metadata and discard errors
                if let Ok(metadata) = path.symlink_metadata() {
                    // Skip this file if it's a symlink
                    if metadata.file_type().is_symlink() {
                        continue;
                    }

                    // Recurse if this is a directory
                    if metadata.file_type().is_dir() {
                        listdirs(&path, output_list);
                    }

                    // Add this to the directory listing if it's a file
                    if metadata.file_type().is_file() {
                        let can_read =
                            OpenOptions::new().read(true).open(&path).is_ok();
                        
                        let can_write =
                            OpenOptions::new().write(true).open(&path).is_ok();

                        output_list.push((path, can_read, can_write));
                    }
                }
            }
        }
    }
}

/// Fuzz thread worker
fn worker(listing: Arc<Vec<(PathBuf, bool, bool)>>, stream: Arc<Mutex<TcpStream>>) {
    // Fuzz buffer
    let mut buf = vec![0x41u8; 32 * 1024];

    let mut blacklist = Vec::new();
    /*blacklist.push("/sys/kernel/debug/ipc_logging");
    blacklist.push("/sys/kernel/debug/kgsl/kgsl-3d0/profiling/pipe");
    blacklist.push("/sys/kernel/debug/tracing");
    blacklist.push("/sys/kernel/debug/smem_log");
    blacklist.push("/sys/devices/virtual/sec/sec_touchkey");
    blacklist.push("/sys/power/wakeup_count");*/
    blacklist.push("/sys/kernel/debug/smp2p_test");
    blacklist.push("/sys/kernel/debug/tracing/trace_pipe");
    blacklist.push("/sys/kernel/debug/tracing/per_cpu");
    blacklist.push("/proc/stlog_pipe");
    blacklist.push("/sys/power/wakeup_count");
    blacklist.push("/sys/kernel/debug/usb_serial0/readstatus");
    blacklist.push("/sys/kernel/debug/usb_serial1/readstatus");
    blacklist.push("/sys/kernel/debug/usb_serial2/readstatus");
    blacklist.push("/sys/kernel/debug/usb_serial3/readstatus");
    blacklist.push("/sys/kernel/debug/mdp/xlog/dump");
    blacklist.push("/sys/kernel/debug/rpm_master_stats");
    blacklist.push("/sys/kernel/debug/mali/job_fault");
    //blacklist.push("/proc/");

    // Fuzz forever
    'next_case: loop {
        let rand_file = rand::random::<usize>() % listing.len();
        let (path, can_read, can_write) = &listing[rand_file];

        if true {
            for thing in &blacklist {
                if path.starts_with(thing) {
                    continue 'next_case;
                }
            }
        }

        if path.starts_with("/proc/") && path.to_str().unwrap().chars().nth(6).unwrap().is_digit(10) {
            continue;
        }

        // Notify server of the file we're fuzzing
        //inform_filename(&stream, path.to_str().unwrap());
        //print!("{:?}\n", path);

        if *can_read {
            // Fuzz by reading
            let fd = OpenOptions::new().read(true).open(path);

            if let Ok(mut fd) = fd {
                let fuzz_size = rand::random::<usize>() % buf.len();
                let _ = fd.read(&mut buf[..fuzz_size]);
            }
        }

        if *can_write {
            // Fuzz by writing
            let fd = OpenOptions::new().write(true).open(path);
            if let Ok(mut fd) = fd {
                let fuzz_size = rand::random::<usize>() % buf.len();
                let _ = fd.write(&buf[..fuzz_size]);
            }
        }
    }
}

fn inform_filename(handle: &Mutex<TcpStream>, filename: &str) {
    // Report the filename
    let mut socket = handle.lock().expect("Failed to lock mutex");
    socket.write_all(filename.as_bytes()).expect("Failed to write");
    socket.flush().expect("Failed to flush");

    // Wait for an ACK
    let mut ack = [0u8; 3];
    socket.read_exact(&mut ack).expect("Failed to read ack");
    assert!(&ack == b"ACK", "Did not get ACK as expected");
}

fn main() {
    // Connect to the server we report to
    let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:13370")
        .expect("Failed to open TCP connection")));

    // Optionally daemonize so we can swap from an ADB USB cable to a UART
    // cable and let this continue to run
    //daemonize();

    // List all files on the system
    let mut dirlisting = Vec::new();
    listdirs(Path::new("/sys/kernel"), &mut dirlisting);

    print!("Created listing of {} files\n", dirlisting.len());

    // We wouldn't do anything without any files
    assert!(dirlisting.len() > 0, "Directory listing was empty");

    // Wrap it in an `Arc`
    let dirlisting = Arc::new(dirlisting);

    // Spawn fuzz threads
    let mut threads = Vec::new();
    for _ in 0..MAX_THREADS {
        // Create a unique arc reference for this thread and spawn the thread
        let dirlisting = dirlisting.clone();
        let stream     = stream.clone();
        threads.push(std::thread::spawn(move || worker(dirlisting, stream)));
    }

    // Wait for all threads to complete
    for thread in threads {
        let _ = thread.join();
    }
}

extern {
    fn daemon(nochdir: i32, noclose: i32) -> i32;
}

pub fn daemonize() {
    print!("Daemonizing\n");

    unsafe {
        daemon(0, 0);
    }

    // Sleep to allow a physical cable swap
    std::thread::sleep(std::time::Duration::from_secs(10));
}
