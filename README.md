# rust-pinyin

[![Build Status](https://github.com/mozillazg/rust-pinyin/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mozillazg/rust-pinyin/actions/workflows/ci.yml)

<!-- [![Coverage Status](https://img.shields.io/coveralls/mozillazg/rust-pinyin/master.svg)](https://coveralls.io/github/mozillazg/rust-pinyin) -->
[![Crates.io Version](https://img.shields.io/crates/v/pinyin.svg)](https://crates.io/crates/pinyin)
[![Doc](https://img.shields.io/badge/doc-reference-blue.svg)](https://docs.rs/pinyin/)

汉语拼音转换工具 Rust 版


Installation
------------

Add this to your `Cargo.toml`:

```
[dependencies]
pinyin = "0.7"
```


Documentation
--------------

API documentation can be found here: https://docs.rs/pinyin/


Usage
------

```rust
use pinyin::{ToPinyin, ToPinyinMulti};

fn main() {
    let hans = "中国人";

    // 无声调，输出 zhong guo ren
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.plain());
        }
    }
    println!();

    // 包含声调，输出 zhōng guó rén
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.with_tone());
        }
    }
    println!();

    // 声调用数字表示，输出 zho1ng guo2 re2n
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            print!("{} ", pinyin.with_tone_num());
        }
    }
    println!();

    // 多音字，输出
    // zho1ng zho4ng
    // guo2
    // re2n
    for multi in hans.to_pinyin_multi() {
        if let Some(multi) = multi {
            for pinyin in multi {
                print!("{} ", pinyin.with_tone_num());
            }
            println!();
        }
    }
}
```


Build
------------

```
$ cargo build
```

Test
------------

```
$ cargo test
```

Data
-----

使用来自 [pinyin-data](https://github.com/mozillazg/pinyin-data) 的拼音数据。


Related Projects
-----------------

* [hotoo/pinyin](https://github.com/hotoo/pinyin): 汉语拼音转换工具 Node.js/JavaScript 版。
* [mozillazg/python-pinyin](https://github.com/mozillazg/python-pinyin): 汉语拼音转换工具 Python 版。
* [mozillazg/go-pinyin](https://github.com/mozillazg/go-pinyin): 汉语拼音转换工具 Go 版。
