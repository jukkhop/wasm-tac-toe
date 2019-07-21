// Math functions

use js_sys::Math::random;
use std::ops::Range;

pub fn random_usize(range: Range<usize>) -> usize {
    range.start + (random() * range.end as f64) as usize
}
