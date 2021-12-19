#![feature(c_size_t)]

use num::complex::{Complex32, Complex64};

#[link(name = "sharp")]
extern "C" {
    pub fn n_alm(
        lmax_: ::std::os::raw::c_size_t,
        mmax_: ::std::os::raw::c_size_t,
    ) -> ::std::os::raw::c_size_t;
}

#[link(name = "sharp")]
extern "C" {
    pub fn hmap2alm(
        map: *const f64,
        alm: *mut Complex64,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn hmap2alm_spin(
        map1: *const f64,
        map2: *const f64,
        alm1: *mut Complex64,
        alm2: *mut Complex64,
        spin: ::std::os::raw::c_int,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn alm2hmap(
        alm: *const Complex64,
        map: *mut f64,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn alm2hmap_spin(
        alm1: *const Complex64,
        alm2: *const Complex64,
        map1: *mut f64,
        map2: *mut f64,
        spin: ::std::os::raw::c_int,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn hmap2alm_f(
        map: *const f32,
        alm: *mut Complex32,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn hmap2alm_spin_f(
        map1: *const f32,
        map2: *const f32,
        alm1: *mut Complex32,
        alm2: *mut Complex32,
        spin: ::std::os::raw::c_int,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn alm2hmap_f(
        alm: *const Complex32,
        map: *mut f32,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}

#[link(name = "sharp")]
extern "C" {
    pub fn alm2hmap_spin_f(
        alm1: *const Complex32,
        alm2: *const Complex32,
        map1: *mut f32,
        map2: *mut f32,
        spin: ::std::os::raw::c_int,
        nside: ::std::os::raw::c_size_t,
        lmax: ::std::os::raw::c_size_t,
        mmax: ::std::os::raw::c_size_t,
        add: ::std::os::raw::c_int,
    );
}
