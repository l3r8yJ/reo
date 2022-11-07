// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod common;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn dataizes_simple_sodg() -> Result<()> {
    let tmp = TempDir::new()?;
    File::create(tmp.path().join("foo.g"))?.write_all(
        "
        ADD($ν1);
        BIND(0, $ν1, foo);
        PUT($ν1, ff-ff);
        "
        .as_bytes(),
    )?;
    let elf = tmp.path().join("temp.elf");
    assert_cmd::Command::cargo_bin("reo")
        .unwrap()
        .current_dir(tmp.path())
        .arg("--verbose")
        .arg("compile")
        .arg(format!("--home={}", tmp.path().display()))
        .arg(elf.as_os_str())
        .assert()
        .success();
    assert_cmd::Command::cargo_bin("reo")
        .unwrap()
        .arg("dataize")
        .arg(format!("--elf={}", elf.display()))
        .arg("foo")
        .assert()
        .success()
        .stdout("FF-FF\n");
    Ok(())
}

#[test]
fn dataizes_in_eoc_mode() -> Result<()> {
    let tmp = TempDir::new()?;
    let dir = tmp.path().join(".eoc").join("sodg");
    fsutils::mkdir(
        dir.to_str()
            .context(format!("Broken path {}", dir.display()))?,
    );
    File::create(dir.join("foo.g"))?.write_all(
        "
        ADD($ν1');
        BIND(0, $ν1, foo);
        PUT($ν1, ca-fe);
        "
        .as_bytes(),
    )?;
    let elf = tmp.path().join("temp.elf");
    assert_cmd::Command::cargo_bin("reo")
        .unwrap()
        .current_dir(tmp.path())
        .arg("compile")
        .arg("--eoc")
        .arg(elf.as_os_str())
        .assert()
        .success();
    assert_cmd::Command::cargo_bin("reo")
        .unwrap()
        .current_dir(tmp.path())
        .arg("dataize")
        .arg(format!("--elf={}", elf.display()))
        .arg("foo")
        .assert()
        .success()
        .stdout("CA-FE\n");
    Ok(())
}
