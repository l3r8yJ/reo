// Copyright (c) 2022-2023 Yegor Bugayenko
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

use anyhow::Result;
use glob::glob;
use sodg::Sodg;
use std::path::Path;

pub fn load_everything() -> Result<Sodg> {
    let pack = Path::new("target/runtime.reo");
    let sources = Path::new("../../target/eo/sodg");
    let target = Path::new("target/reo");
    for f in glob(format!("{}/**/*.sodg", sources.display()).as_str())? {
        let src = f?;
        if src.is_dir() {
            continue;
        }
        let rel = src.as_path().strip_prefix(sources)?.with_extension("reo");
        let bin = target.join(rel);
        assert_cmd::Command::cargo_bin("reo")?
            .arg("compile")
            .arg(src.as_os_str())
            .arg(bin.as_os_str())
            .assert()
            .success();
        assert_cmd::Command::cargo_bin("reo")?
            .arg("merge")
            .arg(bin.as_os_str())
            .arg(bin.as_os_str())
            .assert()
            .success();
    }
    Ok(Sodg::load(pack)?)
}
