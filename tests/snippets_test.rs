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
mod runtime;

use crate::runtime::load_everything;
use anyhow::{Context, Result};
use glob::glob;

fn all_apps() -> Result<Vec<String>> {
    let mut apps = Vec::new();
    for f in glob("eo-tests/**/*.eo")? {
        let p = f?;
        let path = p.as_path();
        let app = path
            .to_str()
            .context(format!("Can't get str from '{}'", path.display()))?
            .splitn(2, "/")
            .nth(1)
            .context(format!("Can't take path from '{}'", path.display()))?
            .split(".")
            .collect::<Vec<&str>>()
            .split_last()
            .context(format!("Can't take split_last from '{}'", path.display()))?
            .1
            .join(".")
            .replace("/", ".");
        apps.push(app.to_string());
    }
    Ok(apps)
}

#[test]
#[ignore]
fn deploys_and_runs_all_apps() -> Result<()> {
    let mut uni = load_everything()?;
    for app in all_apps()? {
        let expected = uni.dataize(format!("Φ.{}.expected", app).as_str());
        let actual = uni.dataize(format!("Φ.{}", app).as_str());
        assert_eq!(expected, actual, "{} failed", app);
    }
    Ok(())
}
