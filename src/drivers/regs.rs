use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Register<T>(usize, PhantomData<T>);

impl<T> Register<T> {
    pub const fn new(addr: usize) -> Self {
        Self(addr, PhantomData)
    }

    #[inline(always)]
    pub unsafe fn read(&self) -> usize {
        unsafe { (self.0 as *const usize).read_volatile() }
    }

    #[inline(always)]
    pub unsafe fn write(&self, val: usize) {
        unsafe { (self.0 as *mut usize).write_volatile(val) };
    }
}

pub struct RegisterValue<T> {
    bits: usize,
    _marker: PhantomData<T>,
}

impl<T> RegisterValue<T> {
    pub const fn new(bits: usize) -> Self {
        Self {
            bits,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn set_field(&mut self, field: Field<T>, value: usize) -> &mut Self {
        self.bits = field.set(self.bits, value);
        self
    }

    #[inline(always)]
    pub const fn set_bit(&mut self, bit: Bit<T>) -> &mut Self {
        self.bits = bit.set(self.bits);
        self
    }

    #[inline(always)]
    pub const fn clear_bit(&mut self, bit: Bit<T>) -> &mut Self {
        self.bits = bit.clear(self.bits);
        self
    }

    #[inline(always)]
    pub const fn is_set_field(&self, field: Field<T>) -> bool {
        field.is_set(self.bits)
    }

    #[inline(always)]
    pub const fn is_set_bit(&self, bit: Bit<T>) -> bool {
        bit.is_set(self.bits)
    }

    #[inline(always)]
    pub const fn raw(&self) -> usize {
        self.bits
    }
}

#[derive(Clone, Copy)]
pub struct Field<T> {
    shift: usize,
    width: usize,
    _marker: PhantomData<T>,
}

impl<T> Field<T> {
    pub const fn new(shift: usize, width: usize) -> Self {
        Self {
            shift,
            width,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn mask(&self) -> usize {
        ((1 << self.width) - 1) << self.shift
    }

    #[inline(always)]
    pub const fn is_set(&self, reg: usize) -> bool {
        reg & self.mask() != 0
    }

    #[inline(always)]
    pub const fn set(&self, reg: usize, val: usize) -> usize {
        (reg & !self.mask()) | ((val & ((1 << self.width) - 1)) << self.shift)
    }

    #[inline(always)]
    pub const fn clear(&self, reg: usize) -> usize {
        reg & !self.mask()
    }
}

#[derive(Clone, Copy)]
pub struct Bit<T> {
    shift: usize,
    _marker: PhantomData<T>,
}

impl<T> Bit<T> {
    pub const fn new(shift: usize) -> Self {
        Self {
            shift,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn mask(&self) -> usize {
        1 << self.shift
    }

    #[inline(always)]
    pub const fn is_set(&self, reg: usize) -> bool {
        reg & self.mask() != 0
    }

    #[inline(always)]
    pub const fn set(&self, reg: usize) -> usize {
        reg | self.mask()
    }

    #[inline(always)]
    pub const fn clear(&self, reg: usize) -> usize {
        reg & !self.mask()
    }
}

macro_rules! register {
    ($name:ident, $addr:expr) => {
        pub mod $name {
            use super::*;
            use crate::drivers::regs::Register;

            pub struct TypeLock;
            pub const REG: Register<TypeLock> = Register::new($addr);

            pub unsafe fn read() -> usize { unsafe { REG.read() } }
            pub unsafe fn write(val: usize) { unsafe { REG.write(val) } }
        }
    };

    ($name:ident, $addr:expr, [ $( $kind:ident: $field:ident, offset: $shift:expr $(, width: $width:expr)? );* $(;)? ]) => {
        pub mod $name {
            use super::*;
            use crate::drivers::regs::{Register, RegisterValue};

            pub struct TypeLock;
            pub const REG: Register<TypeLock> = Register::new($addr);
            type Value = RegisterValue<TypeLock>;

            $(
                register!(@item $kind $field, $shift $(, $width)?);
            )*

            #[inline(always)]
            pub unsafe fn read() -> Value {
                Value::new(unsafe { REG.read() })
            }

            #[inline(always)]
            pub unsafe fn write(val: Value) {
                unsafe { REG.write(val.raw()) }
            }

            #[inline(always)]
            pub unsafe fn new_scope<F: FnOnce(&mut Value)>(f: F) {
                unsafe { let mut v = Value::new(0); f(&mut v); write(v); }
            }

            #[inline(always)]
            pub unsafe fn read_modify_write<F: FnOnce(&mut Value)>(f: F) {
                unsafe { let mut v = read(); f(&mut v); write(v); }
            }

            #[inline(always)]
            pub unsafe fn read_raw() -> usize { unsafe { REG.read() } }

            #[inline(always)]
            pub unsafe fn write_raw(val: usize) {
                unsafe { REG.write(val) };
            }
        }
    };

    (@item field $field:ident, $shift:expr, $width:expr) => {
        pub const $field: crate::drivers::regs::Field<TypeLock> = crate::drivers::regs::Field::new($shift, $width);
    };
    (@item bit $field:ident, $shift:expr $(, $width:expr)?) => {
        pub const $field: crate::drivers::regs::Bit<TypeLock> = crate::drivers::regs::Bit::new($shift);
    };
}

pub(super) use register;
