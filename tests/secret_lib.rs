// Copyright 2014 The Gfx-rs Developers.
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

extern crate gfx as gfx_;
pub use self::gfx_ as gfx;

#[derive(Clone, Debug, PartialEq)]
pub struct Res;

impl gfx::Resources for Res {
    type Buffer = ();
    type ArrayBuffer = ();
    type Shader = ();
    type Program = ();
    type FrameBuffer = ();
    type Surface = ();
    type Texture = ();
    type Sampler = ();
}
