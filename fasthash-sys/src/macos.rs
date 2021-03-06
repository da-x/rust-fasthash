/* automatically generated by rust-bindgen */

pub type conditional_type<_If> = _If;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pair<_T1, _T2> {
    pub first: _T1,
    pub second: _T2,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T1>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T2>>,
}
pub type pair_first_type<_T1> = _T1;
pub type pair_second_type<_T2> = _T2;
pub type pair__EnableB = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pair__CheckArgs {
    pub _address: u8,
}
pub type pair__CheckArgsDep = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pair__CheckTupleLikeConstructor {
    pub _address: u8,
}
pub type pair__CheckTLC = u8;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type uint32 = u32;
pub type uint64 = u64;
pub type uint128 = pair<uint64, uint64>;
extern "C" {
    #[link_name = "\u{1}__Z18CityHash32WithSeedPKcmj"]
    pub fn CityHash32WithSeed(
        buf: *const ::std::os::raw::c_char,
        len: usize,
        seed: uint32,
    ) -> uint32;
}
extern "C" {
    #[link_name = "\u{1}__Z10CityHash64PKcm"]
    pub fn CityHash64(buf: *const ::std::os::raw::c_char, len: usize) -> uint64;
}
extern "C" {
    #[link_name = "\u{1}__Z18CityHash64WithSeedPKcmy"]
    pub fn CityHash64WithSeed(
        buf: *const ::std::os::raw::c_char,
        len: usize,
        seed: uint64,
    ) -> uint64;
}
extern "C" {
    #[link_name = "\u{1}__Z19CityHash64WithSeedsPKcmyy"]
    pub fn CityHash64WithSeeds(
        buf: *const ::std::os::raw::c_char,
        len: usize,
        seed0: uint64,
        seed1: uint64,
    ) -> uint64;
}
extern "C" {
    #[link_name = "\u{1}__Z11CityHash128PKcm"]
    pub fn CityHash128(s: *const ::std::os::raw::c_char, len: usize) -> uint128;
}
extern "C" {
    #[link_name = "\u{1}__Z19CityHash128WithSeedPKcmNSt3__14pairIyyEE"]
    pub fn CityHash128WithSeed(
        s: *const ::std::os::raw::c_char,
        len: usize,
        seed: uint128,
    ) -> uint128;
}
extern "C" {
    #[link_name = "\u{1}__Z14CityHashCrc128PKcm"]
    pub fn CityHashCrc128(s: *const ::std::os::raw::c_char, len: usize) -> uint128;
}
extern "C" {
    #[link_name = "\u{1}__Z22CityHashCrc128WithSeedPKcmNSt3__14pairIyyEE"]
    pub fn CityHashCrc128WithSeed(
        s: *const ::std::os::raw::c_char,
        len: usize,
        seed: uint128,
    ) -> uint128;
}
extern "C" {
    #[link_name = "\u{1}__Z14CityHashCrc256PKcmPy"]
    pub fn CityHashCrc256(s: *const ::std::os::raw::c_char, len: usize, result: *mut uint64);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uint128_c_t {
    pub a: u64,
    pub b: u64,
}
#[test]
fn bindgen_test_layout_uint128_c_t() {
    assert_eq!(
        ::std::mem::size_of::<uint128_c_t>(),
        16usize,
        concat!("Size of: ", stringify!(uint128_c_t))
    );
    assert_eq!(
        ::std::mem::align_of::<uint128_c_t>(),
        8usize,
        concat!("Alignment of ", stringify!(uint128_c_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uint128_c_t>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(uint128_c_t),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uint128_c_t>())).b as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(uint128_c_t),
            "::",
            stringify!(b)
        )
    );
}
extern "C" {
    pub fn farmhash(s: *const ::std::os::raw::c_char, len: usize) -> usize;
}
extern "C" {
    pub fn farmhash32(s: *const ::std::os::raw::c_char, len: usize) -> u32;
}
extern "C" {
    pub fn farmhash32_with_seed(s: *const ::std::os::raw::c_char, len: usize, seed: u32) -> u32;
}
extern "C" {
    pub fn farmhash64(s: *const ::std::os::raw::c_char, len: usize) -> u64;
}
extern "C" {
    pub fn farmhash64_with_seed(s: *const ::std::os::raw::c_char, len: usize, seed: u64) -> u64;
}
extern "C" {
    pub fn farmhash64_with_seeds(
        s: *const ::std::os::raw::c_char,
        len: usize,
        seed0: u64,
        seed1: u64,
    ) -> u64;
}
extern "C" {
    pub fn farmhash128(s: *const ::std::os::raw::c_char, len: usize) -> uint128_c_t;
}
extern "C" {
    pub fn farmhash128_with_seed(
        s: *const ::std::os::raw::c_char,
        len: usize,
        seed: uint128_c_t,
    ) -> uint128_c_t;
}
extern "C" {
    pub fn farmhash_fingerprint32(s: *const ::std::os::raw::c_char, len: usize) -> u32;
}
extern "C" {
    pub fn farmhash_fingerprint64(s: *const ::std::os::raw::c_char, len: usize) -> u64;
}
extern "C" {
    pub fn farmhash_fingerprint128(s: *const ::std::os::raw::c_char, len: usize) -> uint128_c_t;
}
extern "C" {
    #[link_name = "\u{1}__Z13metrohash64_1PKhyjPh"]
    pub fn metrohash64_1(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z13metrohash64_2PKhyjPh"]
    pub fn metrohash64_2(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z16metrohash64crc_1PKhyjPh"]
    pub fn metrohash64crc_1(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z16metrohash64crc_2PKhyjPh"]
    pub fn metrohash64crc_2(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z14metrohash128_1PKhyjPh"]
    pub fn metrohash128_1(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z14metrohash128_2PKhyjPh"]
    pub fn metrohash128_2(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z17metrohash128crc_1PKhyjPh"]
    pub fn metrohash128crc_1(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z17metrohash128crc_2PKhyjPh"]
    pub fn metrohash128crc_2(key: *const u8, len: u64, seed: u32, out: *mut u8);
}
extern "C" {
    #[link_name = "\u{1}__Z11MurmurHash1PKvij"]
    pub fn MurmurHash1(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z18MurmurHash1AlignedPKvij"]
    pub fn MurmurHash1Aligned(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z11MurmurHash2PKvij"]
    pub fn MurmurHash2(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z13MurmurHash64APKviy"]
    pub fn MurmurHash64A(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u64,
    ) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z13MurmurHash64BPKviy"]
    pub fn MurmurHash64B(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u64,
    ) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z12MurmurHash2APKvij"]
    pub fn MurmurHash2A(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z18MurmurHashNeutral2PKvij"]
    pub fn MurmurHashNeutral2(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z18MurmurHashAligned2PKvij"]
    pub fn MurmurHashAligned2(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z18MurmurHash3_x86_32PKvijPv"]
    pub fn MurmurHash3_x86_32(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
        out: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z19MurmurHash3_x86_128PKvijPv"]
    pub fn MurmurHash3_x86_128(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
        out: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z19MurmurHash3_x64_128PKvijPv"]
    pub fn MurmurHash3_x64_128(
        key: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        seed: u32,
        out: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union t1ha_state256 {
    pub bytes: [u8; 32usize],
    pub u32: [u32; 8usize],
    pub u64: [u64; 4usize],
    pub n: t1ha_state256__bindgen_ty_1,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t1ha_state256__bindgen_ty_1 {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}
#[test]
fn bindgen_test_layout_t1ha_state256__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<t1ha_state256__bindgen_ty_1>(),
        32usize,
        concat!("Size of: ", stringify!(t1ha_state256__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<t1ha_state256__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(t1ha_state256__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256__bindgen_ty_1>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256__bindgen_ty_1>())).b as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256__bindgen_ty_1>())).c as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256__bindgen_ty_1),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256__bindgen_ty_1>())).d as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256__bindgen_ty_1),
            "::",
            stringify!(d)
        )
    );
}
#[test]
fn bindgen_test_layout_t1ha_state256() {
    assert_eq!(
        ::std::mem::size_of::<t1ha_state256>(),
        32usize,
        concat!("Size of: ", stringify!(t1ha_state256))
    );
    assert_eq!(
        ::std::mem::align_of::<t1ha_state256>(),
        8usize,
        concat!("Alignment of ", stringify!(t1ha_state256))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256>())).bytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256),
            "::",
            stringify!(bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256>())).u32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256>())).u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256),
            "::",
            stringify!(u64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_state256>())).n as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_state256),
            "::",
            stringify!(n)
        )
    );
}
pub type t1ha_state256_t = t1ha_state256;
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct t1ha_context {
    pub state: t1ha_state256_t,
    pub buffer: t1ha_state256_t,
    pub partial: usize,
    pub total: u64,
    pub __bindgen_padding_0: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_t1ha_context() {
    assert_eq!(
        ::std::mem::size_of::<t1ha_context>(),
        96usize,
        concat!("Size of: ", stringify!(t1ha_context))
    );
    assert_eq!(
        ::std::mem::align_of::<t1ha_context>(),
        32usize,
        concat!("Alignment of ", stringify!(t1ha_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_context>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_context),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_context>())).buffer as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_context),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_context>())).partial as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_context),
            "::",
            stringify!(partial)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t1ha_context>())).total as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(t1ha_context),
            "::",
            stringify!(total)
        )
    );
}
pub type t1ha_context_t = t1ha_context;
extern "C" {
    pub fn t1ha2_atonce(data: *const ::std::os::raw::c_void, length: usize, seed: u64) -> u64;
}
extern "C" {
    pub fn t1ha2_atonce128(
        extra_result: *mut u64,
        data: *const ::std::os::raw::c_void,
        length: usize,
        seed: u64,
    ) -> u64;
}
extern "C" {
    pub fn t1ha2_init(ctx: *mut t1ha_context_t, seed_x: u64, seed_y: u64);
}
extern "C" {
    pub fn t1ha2_update(
        ctx: *mut t1ha_context_t,
        data: *const ::std::os::raw::c_void,
        length: usize,
    );
}
extern "C" {
    pub fn t1ha2_final(ctx: *mut t1ha_context_t, extra_result: *mut u64) -> u64;
}
extern "C" {
    pub fn t1ha1_le(data: *const ::std::os::raw::c_void, length: usize, seed: u64) -> u64;
}
extern "C" {
    pub fn t1ha1_be(data: *const ::std::os::raw::c_void, length: usize, seed: u64) -> u64;
}
pub type t1ha0_function_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const ::std::os::raw::c_void, arg2: usize, arg3: u64) -> u64,
>;
extern "C" {
    pub fn t1ha0_resolve() -> t1ha0_function_t;
}
pub const XXH_errorcode_XXH_OK: XXH_errorcode = 0;
pub const XXH_errorcode_XXH_ERROR: XXH_errorcode = 1;
pub type XXH_errorcode = u32;
extern "C" {
    pub fn fasthash_XXH_versionNumber() -> ::std::os::raw::c_uint;
}
pub type XXH32_hash_t = u32;
extern "C" {
    #[doc = " fasthash_XXH32() :"]
    #[doc = "Calculate the 32-bit hash of sequence \"length\" bytes stored at memory address \"input\"."]
    #[doc = "The memory between input & input+length must be valid (allocated and read-accessible)."]
    #[doc = "\"seed\" can be used to alter the result predictably."]
    #[doc = "Speed on Core 2 Duo @ 3 GHz (single thread, SMHasher benchmark) : 5.4 GB/s"]
    pub fn fasthash_XXH32(
        input: *const ::std::os::raw::c_void,
        length: usize,
        seed: ::std::os::raw::c_uint,
    ) -> XXH32_hash_t;
}
pub type XXH32_state_t = XXH32_state_s;
extern "C" {
    pub fn fasthash_XXH32_createState() -> *mut XXH32_state_t;
}
extern "C" {
    pub fn fasthash_XXH32_freeState(statePtr: *mut XXH32_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH32_copyState(dst_state: *mut XXH32_state_t, src_state: *const XXH32_state_t);
}
extern "C" {
    pub fn fasthash_XXH32_reset(statePtr: *mut XXH32_state_t, seed: ::std::os::raw::c_uint)
        -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH32_update(
        statePtr: *mut XXH32_state_t,
        input: *const ::std::os::raw::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH32_digest(statePtr: *const XXH32_state_t) -> XXH32_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH32_canonical_t {
    pub digest: [::std::os::raw::c_uchar; 4usize],
}
#[test]
fn bindgen_test_layout_XXH32_canonical_t() {
    assert_eq!(
        ::std::mem::size_of::<XXH32_canonical_t>(),
        4usize,
        concat!("Size of: ", stringify!(XXH32_canonical_t))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH32_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH32_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn fasthash_XXH32_canonicalFromHash(dst: *mut XXH32_canonical_t, hash: XXH32_hash_t);
}
extern "C" {
    pub fn fasthash_XXH32_hashFromCanonical(src: *const XXH32_canonical_t) -> XXH32_hash_t;
}
pub type XXH64_hash_t = u64;
extern "C" {
    #[doc = " fasthash_XXH64() :"]
    #[doc = "Calculate the 64-bit hash of sequence of length \"len\" stored at memory address \"input\"."]
    #[doc = "\"seed\" can be used to alter the result predictably."]
    #[doc = "This function runs faster on 64-bit systems, but slower on 32-bit systems (see benchmark)."]
    pub fn fasthash_XXH64(
        input: *const ::std::os::raw::c_void,
        length: usize,
        seed: ::std::os::raw::c_ulonglong,
    ) -> XXH64_hash_t;
}
pub type XXH64_state_t = XXH64_state_s;
extern "C" {
    pub fn fasthash_XXH64_createState() -> *mut XXH64_state_t;
}
extern "C" {
    pub fn fasthash_XXH64_freeState(statePtr: *mut XXH64_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH64_copyState(dst_state: *mut XXH64_state_t, src_state: *const XXH64_state_t);
}
extern "C" {
    pub fn fasthash_XXH64_reset(
        statePtr: *mut XXH64_state_t,
        seed: ::std::os::raw::c_ulonglong,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const ::std::os::raw::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn fasthash_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH64_canonical_t {
    pub digest: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_XXH64_canonical_t() {
    assert_eq!(
        ::std::mem::size_of::<XXH64_canonical_t>(),
        8usize,
        concat!("Size of: ", stringify!(XXH64_canonical_t))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH64_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH64_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn fasthash_XXH64_canonicalFromHash(dst: *mut XXH64_canonical_t, hash: XXH64_hash_t);
}
extern "C" {
    pub fn fasthash_XXH64_hashFromCanonical(src: *const XXH64_canonical_t) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub v1: XXH32_hash_t,
    pub v2: XXH32_hash_t,
    pub v3: XXH32_hash_t,
    pub v4: XXH32_hash_t,
    pub mem32: [XXH32_hash_t; 4usize],
    pub memsize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}
#[test]
fn bindgen_test_layout_XXH32_state_s() {
    assert_eq!(
        ::std::mem::size_of::<XXH32_state_s>(),
        48usize,
        concat!("Size of: ", stringify!(XXH32_state_s))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH32_state_s>(),
        4usize,
        concat!("Alignment of ", stringify!(XXH32_state_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).total_len_32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(total_len_32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).large_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(large_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).v1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(v1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).v2 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(v2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).v3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(v3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).v4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(v4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).mem32 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(mem32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).memsize as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(memsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH32_state_s>())).reserved as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_state_s),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v1: XXH64_hash_t,
    pub v2: XXH64_hash_t,
    pub v3: XXH64_hash_t,
    pub v4: XXH64_hash_t,
    pub mem64: [XXH64_hash_t; 4usize],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
#[test]
fn bindgen_test_layout_XXH64_state_s() {
    assert_eq!(
        ::std::mem::size_of::<XXH64_state_s>(),
        88usize,
        concat!("Size of: ", stringify!(XXH64_state_s))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH64_state_s>(),
        8usize,
        concat!("Alignment of ", stringify!(XXH64_state_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).total_len as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(total_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).v1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(v1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).v2 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(v2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).v3 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(v3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).v4 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(v4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).mem64 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(mem64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).memsize as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(memsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).reserved32 as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(reserved32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH64_state_s>())).reserved64 as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_state_s),
            "::",
            stringify!(reserved64)
        )
    );
}
extern "C" {
    pub fn XXH3_64bits(data: *const ::std::os::raw::c_void, len: usize) -> XXH64_hash_t;
}
extern "C" {
    pub fn XXH3_64bits_withSecret(
        data: *const ::std::os::raw::c_void,
        len: usize,
        secret: *const ::std::os::raw::c_void,
        secretSize: usize,
    ) -> XXH64_hash_t;
}
extern "C" {
    pub fn XXH3_64bits_withSeed(
        data: *const ::std::os::raw::c_void,
        len: usize,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
}
pub type XXH3_state_t = XXH3_state_s;
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct XXH3_state_s {
    pub acc: [XXH64_hash_t; 8usize],
    pub customSecret: [::std::os::raw::c_char; 192usize],
    pub buffer: [::std::os::raw::c_char; 256usize],
    pub bufferedSize: XXH32_hash_t,
    pub nbStripesPerBlock: XXH32_hash_t,
    pub nbStripesSoFar: XXH32_hash_t,
    pub secretLimit: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved32_2: XXH32_hash_t,
    pub totalLen: XXH64_hash_t,
    pub seed: XXH64_hash_t,
    pub reserved64: XXH64_hash_t,
    pub secret: *const ::std::os::raw::c_void,
    pub __bindgen_padding_0: u64,
}
#[test]
fn bindgen_test_layout_XXH3_state_s() {
    assert_eq!(
        ::std::mem::size_of::<XXH3_state_s>(),
        576usize,
        concat!("Size of: ", stringify!(XXH3_state_s))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH3_state_s>(),
        64usize,
        concat!("Alignment of ", stringify!(XXH3_state_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).acc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(acc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).customSecret as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(customSecret)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).buffer as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).bufferedSize as *const _ as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(bufferedSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).nbStripesPerBlock as *const _ as usize },
        516usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(nbStripesPerBlock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).nbStripesSoFar as *const _ as usize },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(nbStripesSoFar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).secretLimit as *const _ as usize },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(secretLimit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).reserved32 as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(reserved32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).reserved32_2 as *const _ as usize },
        532usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(reserved32_2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).totalLen as *const _ as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(totalLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).seed as *const _ as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(seed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).reserved64 as *const _ as usize },
        552usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(reserved64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH3_state_s>())).secret as *const _ as usize },
        560usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH3_state_s),
            "::",
            stringify!(secret)
        )
    );
}
extern "C" {
    pub fn XXH3_createState() -> *mut XXH3_state_t;
}
extern "C" {
    pub fn XXH3_freeState(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_copyState(dst_state: *mut XXH3_state_t, src_state: *const XXH3_state_t);
}
extern "C" {
    pub fn XXH3_64bits_reset(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_reset_withSeed(
        statePtr: *mut XXH3_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_reset_withSecret(
        statePtr: *mut XXH3_state_t,
        secret: *const ::std::os::raw::c_void,
        secretSize: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_update(
        statePtr: *mut XXH3_state_t,
        input: *const ::std::os::raw::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_digest(statePtr: *const XXH3_state_t) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH128_hash_t {
    pub low64: XXH64_hash_t,
    pub high64: XXH64_hash_t,
}
#[test]
fn bindgen_test_layout_XXH128_hash_t() {
    assert_eq!(
        ::std::mem::size_of::<XXH128_hash_t>(),
        16usize,
        concat!("Size of: ", stringify!(XXH128_hash_t))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH128_hash_t>(),
        8usize,
        concat!("Alignment of ", stringify!(XXH128_hash_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH128_hash_t>())).low64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_hash_t),
            "::",
            stringify!(low64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH128_hash_t>())).high64 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_hash_t),
            "::",
            stringify!(high64)
        )
    );
}
extern "C" {
    pub fn XXH128(
        data: *const ::std::os::raw::c_void,
        len: usize,
        seed: XXH64_hash_t,
    ) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits(data: *const ::std::os::raw::c_void, len: usize) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_withSeed(
        data: *const ::std::os::raw::c_void,
        len: usize,
        seed: XXH64_hash_t,
    ) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_withSecret(
        data: *const ::std::os::raw::c_void,
        len: usize,
        secret: *const ::std::os::raw::c_void,
        secretSize: usize,
    ) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_reset(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_reset_withSeed(
        statePtr: *mut XXH3_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_reset_withSecret(
        statePtr: *mut XXH3_state_t,
        secret: *const ::std::os::raw::c_void,
        secretSize: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_update(
        statePtr: *mut XXH3_state_t,
        input: *const ::std::os::raw::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_digest(statePtr: *const XXH3_state_t) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH128_isEqual(h1: XXH128_hash_t, h2: XXH128_hash_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn XXH128_cmp(
        h128_1: *const ::std::os::raw::c_void,
        h128_2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH128_canonical_t {
    pub digest: [::std::os::raw::c_uchar; 16usize],
}
#[test]
fn bindgen_test_layout_XXH128_canonical_t() {
    assert_eq!(
        ::std::mem::size_of::<XXH128_canonical_t>(),
        16usize,
        concat!("Size of: ", stringify!(XXH128_canonical_t))
    );
    assert_eq!(
        ::std::mem::align_of::<XXH128_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH128_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XXH128_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn XXH128_canonicalFromHash(dst: *mut XXH128_canonical_t, hash: XXH128_hash_t);
}
extern "C" {
    pub fn XXH128_hashFromCanonical(src: *const XXH128_canonical_t) -> XXH128_hash_t;
}
pub type HHKey = [u64; 4usize];
pub type HHResult128 = [u64; 2usize];
pub type HHResult256 = [u64; 4usize];
extern "C" {
    pub fn HighwayHash64(key: *mut u64, bytes: *const ::std::os::raw::c_char, size: u64) -> u64;
}
extern "C" {
    pub fn HighwayHash64_TargetPortable(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
    ) -> u64;
}
extern "C" {
    pub fn HighwayHash64_TargetSSE41(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
    ) -> u64;
}
extern "C" {
    pub fn HighwayHash64_TargetAVX2(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
    ) -> u64;
}
extern "C" {
    pub fn HighwayHash64_TargetVSX(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
    ) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z7lookup3PKvij"]
    pub fn lookup3(
        key: *const ::std::os::raw::c_void,
        length: ::std::os::raw::c_int,
        initval: u32,
    ) -> u32;
}
extern "C" {
    #[link_name = "\u{1}__Z28farmhash_fingerprint_uint12811uint128_c_t"]
    pub fn farmhash_fingerprint_uint128(x: uint128_c_t) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z27farmhash_fingerprint_uint64y"]
    pub fn farmhash_fingerprint_uint64(x: u64) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z9mum_hash_PKvmy"]
    pub fn mum_hash_(key: *const ::std::os::raw::c_void, len: usize, seed: u64) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z16SpookyHasherHashPKvmPyS1_"]
    pub fn SpookyHasherHash(
        message: *const ::std::os::raw::c_void,
        length: usize,
        hash1: *mut uint64,
        hash2: *mut uint64,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z15SpookyHasherNewv"]
    pub fn SpookyHasherNew() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}__Z16SpookyHasherFreePv"]
    pub fn SpookyHasherFree(h: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}__Z16SpookyHasherInitPvyy"]
    pub fn SpookyHasherInit(h: *mut ::std::os::raw::c_void, seed1: uint64, seed2: uint64);
}
extern "C" {
    #[link_name = "\u{1}__Z18SpookyHasherUpdatePvPKvm"]
    pub fn SpookyHasherUpdate(
        h: *mut ::std::os::raw::c_void,
        message: *const ::std::os::raw::c_void,
        length: usize,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z17SpookyHasherFinalPvPyS0_"]
    pub fn SpookyHasherFinal(
        h: *mut ::std::os::raw::c_void,
        hash1: *mut uint64,
        hash2: *mut uint64,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z8t1ha0_64PKvmy"]
    pub fn t1ha0_64(data: *const ::std::os::raw::c_void, length: usize, seed: u64) -> u64;
}
extern "C" {
    #[link_name = "\u{1}__Z14HighwayHash128PKyPKcyRA2_y"]
    pub fn HighwayHash128(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
        hash: *mut HHResult128,
    );
}
extern "C" {
    #[link_name = "\u{1}__Z14HighwayHash256PKyPKcyRA4_y"]
    pub fn HighwayHash256(
        key: *mut u64,
        bytes: *const ::std::os::raw::c_char,
        size: u64,
        hash: *mut HHResult256,
    );
}
#[test]
fn __bindgen_test_layout_pair_open0_uint64_uint64_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<pair<uint64, uint64>>(),
        16usize,
        concat!(
            "Size of template specialization: ",
            stringify ! ( pair < uint64 , uint64 > )
        )
    );
    assert_eq!(
        ::std::mem::align_of::<pair<uint64, uint64>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify ! ( pair < uint64 , uint64 > )
        )
    );
}
