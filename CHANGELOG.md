# Changelog

## [0.6.0]

* Use hashmap instead of binary search (via [#27]. Thanks [@hanabi1224])
* 无声调相关风格下对结果去重, Fixed [#25] (via [#28]. Thanks [@hanabi1224])
* 增加 Windows CI (via [#29]. Thanks [@hanabi1224])


## [0.5.0] (2019-04-07)

* 使用 [pinyin-data] v0.7.0 的拼音数据


## [0.4.0] (2018-09-01)

* 移除依赖、增强编译性能和速度 (via [#20]. Thanks [@LuoZijun])
* 使用 [pinyin-data] v0.6.1 的拼音数据


## [0.3.0] (2018-04-30)

* 使用 [pinyin-data] v0.5.1 的拼音数据
* 使用 clippy 优化代码
* 最低支持 1.17 版本的 Rust
* 增加 examples
* 更新依赖包版本:
    * `regex`: `~0.1.8` ->`0.2`
    * `phf`: `~0.7.3` -> `0.7`


## [0.2.0] (2017-10-05)

* 修改 ``pinyin`` 函数，由
  ``pub fn pinyin<'a>(s: &'a str, a: &'a Args) -> Vec<Vec<String>>``
  改为
  ``pub fn pinyin(s: &str, a: &Args) -> Vec<Vec<String>>``
* 增加 ``lazy_pinyin`` 函数:
  ``pub fn lazy_pinyin(s: &str, a: &Args) -> Vec<String>``


## [0.1.0] (2017-09-26)

* 改为使用 [pinyin-data](https://github.com/mozillazg/pinyin-data) v0.4.1 的拼音数据


## [0.0.6] (2016-12-29)

* Use `env::var_os` intead of `env!` in build script
  (via [#5](https://github.com/mozillazg/rust-pinyin/pull/5). Thanks
   [@alexcrichton](https://github.com/alexcrichton))

* Drop support for Rust < 1.3.0


## [0.0.5] (2015-11-21)

* Fixed a regression that caused the crate to stop compiling on    
  current rust nightly and beta versions.
  (via [#1](https://github.com/mozillazg/rust-pinyin/pull/1). Thanks
   [@bluss](https://github.com/bluss))
* Drop support for Rust < 1.2.0


## [0.0.4] (2015-09-20)

* test on Rust 1.3
* fix can't run test on Rust nightly


## [0.0.3] (2015-09-18)

* move build.rs and data out of src directory.


## [0.0.2] (2015-08-30)

* 清理代码
* 更新文档


## 0.0.1 (2015-08-27)

* Initial Release

[pinyin-data]: https://github.com/mozillazg/pinyin-data

[#20]: https://github.com/mozillazg/rust-pinyin/pull/20
[#25]: https://github.com/mozillazg/rust-pinyin/issues/25
[#28]: https://github.com/mozillazg/rust-pinyin/pull/28
[@LuoZijun]: https://github.com/LuoZijun
[@hanabi1224]: https://github.com/hanabi1224

[0.0.2]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.1...v0.0.2
[0.0.3]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.2...v0.0.3
[0.0.4]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.3...v0.0.4
[0.0.5]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.4...v0.0.5
[0.0.6]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.5...v0.0.6
[0.1.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.0.6...v0.1.0
[0.2.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.1.0...v0.2.0
[0.3.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.2.0...v0.3.0
[0.4.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.3.0...v0.4.0
[0.5.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.4.0...v0.5.0
[0.6.0]: https://github.com/mozillazg/rust-pinyin/compare/v0.5.0...v0.6.0
