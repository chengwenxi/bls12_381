use crate::Scalar;
use crate::g1::{G1Affine, G1Projective};
use crate::fp::Fp;

use pasta_curves::Fq;
use subtle::Choice;
use pasta_curves::arithmetic::{FieldExt, Group, SqrtRatio, CurveAffine, CurveExt, Coordinates};
use ff::PrimeField;

use subtle::CtOption;

impl Group for Scalar {
    type Scalar = Scalar;

    fn group_zero() -> Self {
        Scalar::zero()
    }

    fn group_add(&mut self, rhs: &Self) {
        *self = &*self + rhs
    }

    fn group_sub(&mut self, rhs: &Self) {
        *self = &*self - rhs
    }

    fn group_scale(&mut self, by: &Self::Scalar) {
        *self = &*self * by
    }
}

impl SqrtRatio for Scalar {
    const T_MINUS1_OVER2: [u64; 4] = [0, 0, 0, 0];

    fn pow_by_t_minus1_over2(&self) -> Self {
        unimplemented!();
    }

    fn get_lower_32(&self) -> u32 {
        unimplemented!();
    }

    #[cfg(feature = "sqrt-table")]
    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        unimplemented!();
    }

    #[cfg(feature = "sqrt-table")]
    fn sqrt_alt(&self) -> (Choice, Self) {
        unimplemented!();
    }
}

impl FieldExt for Scalar {
    const MODULUS: &'static str =
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    //0x0538a6f66e19c653ed4f2f74a35d01686f67d4a2b566f8330fb4d6e13cf19a78
    const ROOT_OF_UNITY_INV: Self = unimplemented!();
    /*Fr([
        0xfb4d6e13cf19a78,
        0x6f67d4a2b566f833,
        0xed4f2f74a35d0168,
        0x538a6f66e19c653
    ]);
    */

    const DELTA: Self = unimplemented!();

    //0x39f6d3a994cebea4199cec0404d0ec02a9ded2017fff2dff7fffffff80000001
    const TWO_INV: Self = unimplemented!();
    /*
    Fr::from_bytes([
        [0x7fffffff80000001,
        0xa9ded2017fff2dff,
        0x199cec0404d0ec02,
        0x39f6d3a994cebea4,
        ].
    ]); */

    const ZETA: Self = unimplemented!();

    fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        Scalar::from_bytes_wide(bytes)
    }

    fn from_u128(_v: u128) -> Self {
        todo!()
    }

    fn get_lower_128(&self) -> u128 {
        todo!()
    }
}

impl Group for Fp {
    type Scalar = Fp;

    fn group_zero() -> Self {
        Fp::zero()
    }

    fn group_add(&mut self, rhs: &Self) {
        *self = &*self + rhs
    }

    fn group_sub(&mut self, rhs: &Self) {
        *self = &*self - rhs
    }

    fn group_scale(&mut self, by: &Self::Scalar) {
        *self = &*self * by
    }
}

impl From<bool> for Fp {
    fn from(b: bool) -> Self {
        if b {
            Fp::one()
        } else {
            Fp::zero()
        }
    }
}

impl From<u64> for Fp {
    fn from(n: u64) -> Self {
        unimplemented!()
    }
}

impl PrimeField for Fp {
    type Repr = [u8; 32];

    fn from_repr(r: Self::Repr) -> CtOption<Self> {
        unimplemented!()
    }

    fn to_repr(&self) -> Self::Repr {
        unimplemented!()
    }

    fn is_odd(&self) -> Choice {
        Choice::from(self.to_bytes()[0] & 1)
    }

    const NUM_BITS: u32 = 383;
    const CAPACITY: u32 = Self::NUM_BITS - 1;

    fn multiplicative_generator() -> Self {
        unimplemented!()
    }

    const S: u32 = 32;

    fn root_of_unity() -> Self {
        unimplemented!()
    }
}

impl SqrtRatio for Fp {
    const T_MINUS1_OVER2: [u64; 4] = [0, 0, 0, 0];

    fn pow_by_t_minus1_over2(&self) -> Self {
        unimplemented!();
    }

    fn get_lower_32(&self) -> u32 {
        unimplemented!();
    }

    #[cfg(feature = "sqrt-table")]
    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        unimplemented!();
    }

    #[cfg(feature = "sqrt-table")]
    fn sqrt_alt(&self) -> (Choice, Self) {
        unimplemented!();
    }
}

impl FieldExt for Fp {
    const MODULUS: &'static str =
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    //0x0538a6f66e19c653ed4f2f74a35d01686f67d4a2b566f8330fb4d6e13cf19a78
    const ROOT_OF_UNITY_INV: Self = unimplemented!();
    /*Fr([
        0xfb4d6e13cf19a78,
        0x6f67d4a2b566f833,
        0xed4f2f74a35d0168,
        0x538a6f66e19c653
    ]);
    */

    const DELTA: Self = unimplemented!();

    //0x39f6d3a994cebea4199cec0404d0ec02a9ded2017fff2dff7fffffff80000001
    const TWO_INV: Self = unimplemented!();
    /*
    Fr::from_bytes([
        [0x7fffffff80000001,
        0xa9ded2017fff2dff,
        0x199cec0404d0ec02,
        0x39f6d3a994cebea4,
        ].
    ]); */

    const ZETA: Self = unimplemented!();

    fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        Fp::from_bytes_wide(bytes)
    }

    fn from_u128(_v: u128) -> Self {
        todo!()
    }

    fn get_lower_128(&self) -> u128 {
        todo!()
    }
}

impl Group for G1Projective {
    type Scalar = Scalar;

    fn group_zero() -> Self {
        G1Projective::identity()
    }

    fn group_add(&mut self, rhs: &Self) {
        *self = &*self + rhs
    }

    fn group_sub(&mut self, rhs: &Self) {
        *self = &*self - rhs
    }

    fn group_scale(&mut self, by: &Self::Scalar) {
        *self = &*self * by
    }
}

impl CurveExt for  G1Projective{
    type ScalarExt = Scalar;

    type Base = Fq;

    type AffineExt = G1Affine;

    const CURVE_ID: &'static str = "bls12_381_g1";

    fn endo(&self) -> Self {
        todo!()
    }

    fn jacobian_coordinates(&self) -> (Self::Base, Self::Base, Self::Base) {
        todo!()
    }

    fn is_on_curve(&self) -> Choice {
        todo!()
    }

    fn a() -> Self::Base {
        todo!()
    }

    fn b() -> Self::Base {
        todo!()
    }

    fn new_jacobian(_x: Self::Base, _y: Self::Base, _z: Self::Base) -> CtOption<Self> {
        todo!()
    }

    fn hash_to_curve<'a>(domain_prefix: &'a str) -> alloc::boxed::Box<dyn Fn(&[u8]) -> Self + 'a> {
        todo!()
    }
}

impl CurveAffine for G1Affine {
    type ScalarExt = Scalar;

    type Base = Fp;

    type CurveExt = G1Projective;

    fn is_on_curve(&self) -> Choice {
        self.is_on_curve()
    }

    fn from_xy(x: Self::Base, y: Self::Base) -> CtOption<Self> {
        let identity = Self::identity();
        if x == identity.x && y == identity.y {
            CtOption::new(identity, Choice::from(1u8))
        } else {
            let mut p = Self::generator();
            p.x = x;
            p.y = y;
            CtOption::new(p, p.is_on_curve())
        }
    }

    fn coordinates(&self) -> CtOption<Coordinates<Self>> {
        Coordinates::from_xy( self.x, self.y )
    }

    fn a() -> Self::Base {
        Fp::zero()
    }

    fn b() -> Self::Base {
        let two = Fp::one() + Fp::one();
        two + two
    }
}