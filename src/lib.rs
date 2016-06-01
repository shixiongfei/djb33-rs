/*
    Copyright (c) 2016 Xiongfei Shi (shixf.com)

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.

*/

pub const DJB33_INIT: u32 = 5381;

pub fn djb33(mut h: u32, data: &[u8]) -> u32 {
    let len = data.len();
    
    if len > 0 {
        for i in 0..len  {
            /* hash * 33 + d */
            h = h.wrapping_mul(33).wrapping_add(data[i] as u32);
        }
    }
    
    h
}

pub fn djb33_xor(mut h: u32, data: &[u8]) -> u32 {
    let len = data.len();
    
    if len > 0 {
        for i in 0..len  {
            /* hash * 33 ^ d */
            h = h.wrapping_mul(33) ^ (data[i] as u32);
        }
    }
    
    h
}
