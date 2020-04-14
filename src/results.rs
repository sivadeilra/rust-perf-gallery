//! Gallery of error-handling patterns.

pub struct SimpleError;

pub enum EnumError {
    A,
    B,
    OutOfMemory,
}

pub enum ErrorWithData {
    StaticStr(&'static str),
    A(i32),
    B,
    C,
}

pub enum ErrorWithDropData {
    String(String),
    StaticStr(&'static str),
    Other,
}

// I'm using macro_rules to generate variations rather than using generics for several reasons.
// First, the compiler will not emit specializations for generics unless you request them, and
// requesting them takes even more effort, since I would need to write the generic function, then
// N different non-generic function that just called the generic function. Second, I don't want the
// compiler to "know" that the different variations come from a single AST source (the generic
// function definition). I want the compiler to have as little data as possible, because I want to
// see what information the compiler can discover about the code.

macro_rules! try_push_variants {
    ($out:ty, $good:expr) => {
        pub fn try_push_returning_bool(v: &mut Vec<u32>, x: u32) -> bool {
            if v.len() < v.capacity() {
                v.push(x);
                true
            } else {
                false
            }
        }

        pub fn try_push_returning_simple_error(
            v: &mut Vec<u32>,
            x: u32,
        ) -> Result<$out, SimpleError> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(SimpleError)
            }
        }

        pub fn try_push_returning_unit(v: &mut Vec<u32>, x: u32) -> Result<$out, ()> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(())
            }
        }

        #[repr(transparent)]
        pub struct HRESULT(pub i32);
        impl HRESULT {
            pub const E_OUTOFMEMORY: HRESULT = HRESULT(0x8007000eu32 as i32);
        }

        /// The Result is returned in the register pair (%eax, %edx), with the enum discriminant in %eax
        /// and the HRESULT value in %edx. Excellent.
        pub fn try_push_returning_hresult(v: &mut Vec<u32>, x: u32) -> Result<$out, HRESULT> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(HRESULT::E_OUTOFMEMORY)
            }
        }

        /// The Result is returned in the register pair (%eax, %edx), with the enum discriminant in %eax
        /// and the HRESULT value in %edx. Excellent.
        ///
        /// This is true even though the NTSTATUS type is not marked #[repr(transparent)].
        pub struct NTSTATUS(pub i32);
        impl NTSTATUS {
            pub const STATUS_NO_MEMORY: NTSTATUS = NTSTATUS(0xc0000017u32 as i32);
        }

        pub fn try_push_returning_ntstatus(v: &mut Vec<u32>, x: u32) -> Result<$out, NTSTATUS> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(NTSTATUS::STATUS_NO_MEMORY)
            }
        }

        pub fn try_push_enum_error(v: &mut Vec<u32>, x: u32) -> Result<$out, EnumError> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(EnumError::OutOfMemory)
            }
        }

        pub fn try_push_returning_enum_error(v: &mut Vec<u32>, x: u32) -> Result<$out, EnumError> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(EnumError::OutOfMemory)
            }
        }

        pub fn try_push_returning_data_static_str(
            v: &mut Vec<u32>,
            x: u32,
        ) -> Result<$out, ErrorWithData> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(ErrorWithData::StaticStr("Out of memory"))
            }
        }

        pub fn try_push_returning_drop_data_string(
            v: &mut Vec<u32>,
            x: u32,
        ) -> Result<$out, ErrorWithDropData> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(ErrorWithDropData::String("Out of memory".to_string()))
            }
        }

        pub fn try_push_returning_drop_data_str(
            v: &mut Vec<u32>,
            x: u32,
        ) -> Result<$out, ErrorWithDropData> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(ErrorWithDropData::StaticStr("Out of memory"))
            }
        }

        pub fn try_push_returning_drop_data_other(
            v: &mut Vec<u32>,
            x: u32,
        ) -> Result<$out, ErrorWithDropData> {
            if v.len() < v.capacity() {
                v.push(x);
                Ok($good)
            } else {
                Err(ErrorWithDropData::Other)
            }
        }
    };
}

pub mod ok_is_unit {
    use super::*;
    try_push_variants!((), ());
}

pub mod ok_is_i32 {
    use super::*;
    try_push_variants!(i32, 333);
}

pub mod ok_is_string {
    use super::*;
    try_push_variants!(String, String::new());
}
