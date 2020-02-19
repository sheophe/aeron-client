/*
 * Copyright 2020 UT OVERSEAS INC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::ffi::CString;

use crate::concurrent::atomic_buffer::AtomicBuffer;
use crate::utils::types::Index;

pub struct Flyweight<T> {
    pub m_struct: T,
    buffer: AtomicBuffer,
    base_offset: Index,
}

impl<T: Copy> Flyweight<T> {
    pub fn new(buffer: AtomicBuffer, base_offset: Index) -> Self {
        Self {
            m_struct: buffer.get::<T>(base_offset),
            buffer,
            base_offset,
        }
    }

    #[inline]
    pub fn string_get(&self, offset: Index) -> CString {
        self.buffer.get_string(offset)
    }

    #[inline]
    pub fn string_get_length(&self, offset: Index) -> Index {
        self.buffer.get_string_length(offset)
    }

    #[inline]
    pub fn string_put(&mut self) {
        unimplemented!()
    }

    #[inline]
    pub fn string_put_without_length(&mut self) {
        unimplemented!()
    }

    #[inline]
    pub fn string_get_without_length(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn put_bytes(&self, offset: Index, src: &[u8]) {
        self.buffer.put_bytes(self.base_offset + offset, src)
    }

    #[inline]
    pub fn get_bytes(&self, _position: Index, _val: T) {
        unimplemented!()
    }

    #[inline]
    pub fn get<U: Copy>(&self, offset: Index) -> U {
        self.buffer.get::<U>(self.base_offset + offset)
    }

    #[inline]
    pub fn put<U>(&self, offset: Index, value: U) {
        self.buffer.put::<U>(self.base_offset + offset, value);
    }
}