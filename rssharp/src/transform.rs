use crate::{
    ffi::{nside2npix, SharpFloat}
    , alm::{
        n_alm
    }
};

use std::{default::Default};

use num::{complex::Complex, traits::Float};

pub fn alm2hmap<T>(alm: &[Complex<T>], nside: usize, lmax: usize, mmax: usize) -> Vec<T>
where
    T: SharpFloat + Float + Default,
{
    let nresult = nside2npix(nside);
    let mut result = vec![T::default(); nresult];
    T::alm2hmap(alm, &mut result, nside, lmax, mmax, false);
    result
}

pub fn alm2hmap_spin<T>(
    alm1: &[Complex<T>],
    alm2: &[Complex<T>],
    spin: i32,
    nside: usize,
    lmax: usize,
    mmax: usize,
) -> (Vec<T>, Vec<T>)
where
    T: SharpFloat + Float + Default,
{
    let nresult = nside2npix(nside);
    let mut map1 = vec![T::default(); nresult];
    let mut map2 = vec![T::default(); nresult];
    T::alm2hmap_spin(
        alm1, alm2, &mut map1, &mut map2, spin, nside, lmax, mmax, false,
    );
    (map1, map2)
}

pub fn hmap2alm_iter<T>(
    map: &[T],
    nside: usize,
    lmax: usize,
    mmax: usize,
    niter: usize,
) -> Vec<Complex<T>>
where
    T: SharpFloat + Float + Default,
{
    let nresult = n_alm(lmax, mmax);
    let mut result = vec![Complex::<T>::default(); nresult];
    let mut map2 = vec![T::default(); map.len()];
    T::hmap2alm(map, &mut result, nside, lmax, mmax, false);
    for _iter in 0..niter {
        T::alm2hmap(&result, &mut map2, nside, lmax, mmax, false);
        for (m2, &m) in &mut map2.iter_mut().zip(map.iter()) {
            *m2 = m - *m2;
        }
        T::hmap2alm(&map2, &mut result, nside, lmax, mmax, true);
    }
    result
}

pub fn alm2hmap_pol<T>(alm_t: &[Complex<T>], alm_g: &[Complex<T>], alm_c: &[Complex<T>], nside: usize, lmax: usize, mmax: usize)->(Vec<T>, Vec<T>, Vec<T>)
where T:SharpFloat+Float+Default,{
    let mut hmap_t=vec![T::default(); nside2npix(nside)];
    let mut hmap_q=vec![T::default(); nside2npix(nside)];
    let mut hmap_u=vec![T::default(); nside2npix(nside)];
    T::alm2hmap_pol(alm_t, alm_g, alm_c, &mut hmap_t, &mut hmap_q, &mut hmap_u, nside, lmax, mmax, false);
    (hmap_t, hmap_q, hmap_u)
}

pub fn hmap2alm_pol_iter<T>(
    map_t: &[T], 
    map_q: &[T],
    map_u: &[T],
    nside: usize, 
    lmax: usize, 
    mmax: usize,
    niter: usize
)->(Vec<Complex<T>>, Vec<Complex<T>>, Vec<Complex<T>>)
where
    T: SharpFloat + Float + Default,
{
    let mut alm_t=vec![Complex::<T>::default(); n_alm(lmax, mmax)];
    let mut alm_g=vec![Complex::<T>::default(); n_alm(lmax, mmax)];
    let mut alm_c=vec![Complex::<T>::default(); n_alm(lmax, mmax)];
    T::hmap2alm_pol(map_t, map_q, map_q, &mut alm_t, &mut alm_g, &mut alm_c, nside, lmax, mmax, false);
    let mut map_t2=vec![T::default(); map_t.len()];
    let mut map_q2=vec![T::default(); map_t.len()];
    let mut map_u2=vec![T::default(); map_t.len()];

    for _niter in 0..niter{
        T::alm2hmap_pol(&alm_t, &alm_g, &alm_c, &mut map_t2, &mut map_q2, &mut map_u2, nside, lmax, mmax, false);
        for (m2,&m) in map_t2.iter_mut().zip(map_t.iter()){
            *m2=m-*m2;
        }
        for (m2,&m) in map_q2.iter_mut().zip(map_q.iter()){
            *m2=m-*m2;
        }
        for (m2,&m) in map_u2.iter_mut().zip(map_u.iter()){
            *m2=m-*m2;
        }
        T::hmap2alm_pol(&map_t2, &map_q2, &map_u2, &mut alm_t, &mut alm_g, &mut alm_c, nside, lmax, mmax, true);
    }

    (alm_t, alm_g, alm_c)
}
