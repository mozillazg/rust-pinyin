# rust-pinyin

[![Build Status](https://img.shields.io/travis/mozillazg/rust-pinyin/master.svg)](https://travis-ci.org/mozillazg/rust-pinyin)
[![Build status](https://ci.appveyor.com/api/projects/status/bemojvswjsqo796s/branch/master?svg=true)](https://ci.appveyor.com/project/mozillazg/rust-pinyin/branch/master)
[![Coverage Status](https://img.shields.io/coveralls/mozillazg/rust-pinyin/master.svg)](https://coveralls.io/github/mozillazg/rust-pinyin)
[![Crates.io Version](https://img.shields.io/crates/v/pinyin.svg)](https://crates.io/crates/pinyin)
[![Doc](https://img.shields.io/badge/doc-reference-blue.svg)](https://docs.rs/pinyin/)

汉语拼音转换工具 Rust 版


Installation
------------

Add this to your `Cargo.toml`:

```
[dependencies]
pinyin = "0.5"
```

and this to your crate root:

```
extern crate pinyin;
```


Documentation
--------------

API documentation can be found here: https://docs.rs/pinyin/


Usage
------

```rust
extern crate pinyin;

pub fn main() {
    let hans = "中国人";
    let mut args = pinyin::Args::new();

    // 默认输出 [["zhong"] ["guo"] ["ren"]]
    println!("{:?}",  pinyin::pinyin(hans, &args));
    // ["zhong", "guo", "ren"]
    println!("{:?}",  pinyin::lazy_pinyin(hans, &args));

    // 包含声调 [["zhōng"], ["guó"], ["rén"]]
    args.style = pinyin::Style::Tone;
    println!("{:?}",  pinyin::pinyin(hans, &args));

    // 声调用数字表示 [["zho1ng"] ["guo2"] ["re2n"]]
    args.style = pinyin::Style::Tone2;
    println!("{:?}",  pinyin::pinyin(hans, &args));

    // 开启多音字模式
    args = pinyin::Args::new();
    args.heteronym = true;
    // [["zhong"] ["guo"] ["ren"]]
    println!("{:?}",  pinyin::pinyin(hans, &args));

    // [["zho1ng", "zho4ng"] ["guo2"] ["re2n"]]
    args.style = pinyin::Style::Tone2;
    println!("{:?}",  pinyin::pinyin(hans, &args));
}
```


Build
------------

```
$ make build
```

Test
------------

```
$ make test
```

Data
-----

使用来自 [pinyin-data](https://github.com/mozillazg/pinyin-data) 的拼音数据。


Related Projects
-----------------

* [hotoo/pinyin](https://github.com/hotoo/pinyin): 汉语拼音转换工具 Node.js/JavaScript 版。
* [mozillazg/python-pinyin](https://github.com/mozillazg/python-pinyin): 汉语拼音转换工具 Python 版。
* [mozillazg/go-pinyin](https://github.com/mozillazg/go-pinyin): 汉语拼音转换工具 Go 版。
