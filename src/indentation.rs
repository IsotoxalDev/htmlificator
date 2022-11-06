// MIT License
// 
// Copyright (c) 2016 Martin Geisler
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


// Code Snippet from https://github.com/mgeisler/textwrap/
pub fn indent(s: &str, prefix: &str) -> String {
    let mut result = String::with_capacity(2 * s.len());
    let trimmed_prefix = prefix.trim_end();
    for (idx, line) in s.split_terminator('\n').enumerate() {
        if idx > 0 {
            result.push('\n');
        }
        if line.trim().is_empty() {
            result.push_str(trimmed_prefix);
        } else {
            result.push_str(prefix);
        }
        result.push_str(line);
    }
    if s.ends_with('\n') {
        result.push('\n');
    }
    result
}