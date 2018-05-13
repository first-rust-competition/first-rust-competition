/* THE ORIGINAL VERSION OF THIS FILE WAS DISTRIBUTED WITH THE FOLLOWING LICENSE

"""
MIT License
Copyright (c) 2017 Rust for Robotics Developers
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

THE MODIFIED FORM OF THIS FILE IS LICENSED UNDER THE SAME TERMS AS THE REST OF THIS REPOSITORY.
SEE THE LICENSE FILE FOR FULL TERMS.
*/

/// Handles only doing some task once per set interval.
pub struct Throttler<T, S> {
    next_send: T,
    interval: S,
}

impl Throttler<u64, u64> {
    /// Create a new throttler.
    pub fn new(now: u64, interval: u64) -> Throttler<u64, u64> {
        Throttler {
            next_send: now + interval,
            interval: interval,
        }
    }

    /// Update the throttler. Returns true if the task should be performed.
    pub fn update(&mut self, now: u64) -> bool {
        if now > self.next_send {
            self.next_send = self.next_send + self.interval;
            true
        } else {
            false
        }
    }
}
