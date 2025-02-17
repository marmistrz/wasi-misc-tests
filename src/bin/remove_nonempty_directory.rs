use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_dir, create_dir};
use misc_tests::wasi::wasi_path_remove_directory;
use std::{env, process};

fn test_remove_nonempty_directory(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    create_dir(dir_fd, "dir");

    // Create a directory in the directory we just created.
    create_dir(dir_fd, "dir/nested");

    // Test that attempting to unlink the first directory returns the expected error code.
    let mut status = wasi_path_remove_directory(dir_fd, "dir");
    assert_eq!(
        status,
        libc::__WASI_ENOTEMPTY,
        "remove_directory on a directory should return ENOTEMPTY",
    );

    // Removing the directories.
    status = wasi_path_remove_directory(dir_fd, "dir/nested");
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a nested directory should succeed",
    );
    cleanup_dir(dir_fd, "dir");
}

fn main() {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    // Open scratch directory
    let dir_fd = match open_scratch_directory(&arg) {
        Ok(dir_fd) => dir_fd,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };

    // Run the tests.
    test_remove_nonempty_directory(dir_fd)
}
