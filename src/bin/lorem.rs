// Copyright 2020 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use diesel_demo::schema::posts::dsl::*;
use lipsum::{lipsum, lipsum_title};

fn main() {
    const POST_NUM: i32 = 10;
    let connection = establish_connection();
    for _ in 0..POST_NUM {
        let post = create_post(&connection, &lipsum_title(), &lipsum(25));
        println!("\n Saved draft {} with id {}", post.title, post.id);
    }
    diesel::update(posts)
        .set(published.eq(true))
        .execute(&connection).expect("Error Update posts");
}
