// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for `int`

#[allow(non_uppercase_statics)];

use prelude::*;

use default::Default;
use num::{BitCount, CheckedAdd, CheckedSub, CheckedMul};
use num::{CheckedDiv, Zero, One, strconv};
use num::{ToStrRadix, FromStrRadix};
use option::{Option, Some, None};
use str;
use unstable::intrinsics;

#[cfg(target_word_size = "32")] int_module!(int, 32)
#[cfg(target_word_size = "64")] int_module!(int, 64)

#[cfg(target_word_size = "32")]
impl BitCount for int {
    /// Counts the number of bits set. Wraps LLVM's `ctpop` intrinsic.
    #[inline]
    fn population_count(&self) -> int { (*self as i32).population_count() as int }

    /// Counts the number of leading zeros. Wraps LLVM's `ctlz` intrinsic.
    #[inline]
    fn leading_zeros(&self) -> int { (*self as i32).leading_zeros() as int }

    /// Counts the number of trailing zeros. Wraps LLVM's `cttz` intrinsic.
    #[inline]
    fn trailing_zeros(&self) -> int { (*self as i32).trailing_zeros() as int }
}

#[cfg(target_word_size = "64")]
impl BitCount for int {
    /// Counts the number of bits set. Wraps LLVM's `ctpop` intrinsic.
    #[inline]
    fn population_count(&self) -> int { (*self as i64).population_count() as int }

    /// Counts the number of leading zeros. Wraps LLVM's `ctlz` intrinsic.
    #[inline]
    fn leading_zeros(&self) -> int { (*self as i64).leading_zeros() as int }

    /// Counts the number of trailing zeros. Wraps LLVM's `cttz` intrinsic.
    #[inline]
    fn trailing_zeros(&self) -> int { (*self as i64).trailing_zeros() as int }
}

#[cfg(target_word_size = "32")]
impl CheckedAdd for int {
    #[inline]
    fn checked_add(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i32_add_with_overflow(*self as i32, *v as i32);
            if y { None } else { Some(x as int) }
        }
    }
}

#[cfg(target_word_size = "64")]
impl CheckedAdd for int {
    #[inline]
    fn checked_add(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i64_add_with_overflow(*self as i64, *v as i64);
            if y { None } else { Some(x as int) }
        }
    }
}

#[cfg(target_word_size = "32")]
impl CheckedSub for int {
    #[inline]
    fn checked_sub(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i32_sub_with_overflow(*self as i32, *v as i32);
            if y { None } else { Some(x as int) }
        }
    }
}

#[cfg(target_word_size = "64")]
impl CheckedSub for int {
    #[inline]
    fn checked_sub(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i64_sub_with_overflow(*self as i64, *v as i64);
            if y { None } else { Some(x as int) }
        }
    }
}

#[cfg(target_word_size = "32")]
impl CheckedMul for int {
    #[inline]
    fn checked_mul(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i32_mul_with_overflow(*self as i32, *v as i32);
            if y { None } else { Some(x as int) }
        }
    }
}

#[cfg(target_word_size = "64")]
impl CheckedMul for int {
    #[inline]
    fn checked_mul(&self, v: &int) -> Option<int> {
        unsafe {
            let (x, y) = intrinsics::i64_mul_with_overflow(*self as i64, *v as i64);
            if y { None } else { Some(x as int) }
        }
    }
}

/// Returns `base` raised to the power of `exponent`
pub fn pow(base: int, exponent: uint) -> int {
    if exponent == 0u {
        //Not mathemtically true if ~[base == 0]
        return 1;
    }
    if base == 0 { return 0; }
    let mut my_pow  = exponent;
    let mut acc     = 1;
    let mut multiplier = base;
    while(my_pow > 0u) {
        if my_pow % 2u == 1u {
            acc *= multiplier;
        }
        my_pow     /= 2u;
        multiplier *= multiplier;
    }
    return acc;
}

#[test]
fn test_pow() {
    assert!((pow(0, 0u) == 1));
    assert!((pow(0, 1u) == 0));
    assert!((pow(0, 2u) == 0));
    assert!((pow(-1, 0u) == 1));
    assert!((pow(1, 0u) == 1));
    assert!((pow(-3, 2u) == 9));
    assert!((pow(-3, 3u) == -27));
    assert!((pow(4, 9u) == 262144));
}

#[test]
fn test_overflows() {
    assert!((::int::max_value > 0));
    assert!((::int::min_value <= 0));
    assert!((::int::min_value + ::int::max_value + 1 == 0));
}
