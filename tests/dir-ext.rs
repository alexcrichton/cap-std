mod sys_common;

use cap_fs_ext::DirExt;
use cap_tempfile::{ambient_authority, TempDir};

use sys_common::symlink_supported;

#[test]
fn remove_file() {
    let tempdir = TempDir::new(ambient_authority()).expect("create tempdir");
    let file = tempdir.create("file").expect("create file to delete");
    drop(file);
    tempdir.remove_file_or_symlink("file").expect("delete file");
    assert!(!tempdir.exists("file"), "deletion worked");
}

#[test]
fn remove_symlink_to_file() {
    if !symlink_supported() {
        return;
    }

    let tempdir = TempDir::new(ambient_authority()).expect("create tempdir");
    let target = tempdir.create("target").expect("create target file");
    drop(target);
    tempdir.symlink("target", "link").expect("create symlink");
    assert!(tempdir.exists("link"), "link exists");
    tempdir
        .remove_file_or_symlink("link")
        .expect("delete symlink");
    assert!(!tempdir.exists("link"), "link deleted");
    assert!(tempdir.exists("target"), "target not deleted");
}

#[test]
fn remove_symlink_to_dir() {
    if !symlink_supported() {
        return;
    }

    let tempdir = TempDir::new(ambient_authority()).expect("create tempdir");
    {
        let _target = tempdir.create_dir("target").expect("create target dir");
    }
    tempdir.symlink("target", "link").expect("create symlink");
    assert!(tempdir.exists("link"), "link exists");
    tempdir
        .remove_file_or_symlink("link")
        .expect("delete symlink");
    assert!(!tempdir.exists("link"), "link deleted");
    assert!(tempdir.exists("target"), "target not deleted");
}

#[test]
fn do_not_remove_dir() {
    let tempdir = TempDir::new(ambient_authority()).expect("create tempdir");
    {
        let _subdir = tempdir.create_dir("subdir").expect("create dir");
    }
    assert!(tempdir.exists("subdir"), "subdir created");
    tempdir
        .remove_file_or_symlink("subdir")
        .expect_err("should not delete empty directory");
    assert!(tempdir.exists("subdir"), "subdir not deleted");
}
