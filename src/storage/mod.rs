// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Implementation of different persistent storages. Currently on disk (`FileStorage`) and in Redis
//! (`RedisStorage`).

/// `RedisStorage`
pub mod redis;
/// `FileStorage`
pub mod file;

/// Bring them into scope
pub use storage::redis::RedisStorage;
pub use storage::file::FileStorage;