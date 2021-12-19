extern crate rssharp;

use num::{
    complex::{
        Complex
    }
};

use std::{
    io::Write
    , fs::File
};
use rssharp::{
    transform
    , ffi::{
        nside2npix
        , SharpFloat
    }
    , alm::{
        n_alm
    }
};

fn main() {
    let nside=16;
    let npix=nside2npix(nside);
    let lmax=nside*2-1;
    let mmax=lmax;

    let mut map1=vec![1.0; npix];
    let mut map2:Vec<_>=(0..npix).map(|i| (i%2) as i32 as f64).collect();

    let mut alm1=vec![Complex::<f64>::new(0.0, 0.0); n_alm(lmax, mmax)];
    let mut alm2=vec![Complex::<f64>::new(0.0, 0.0); n_alm(lmax, mmax)];
    //let (alm1,alm2)=transform::hmap2alm_spin_iter(&map1, &map2,2, nside, lmax, mmax, 1e-3,1e-3);
    f64::hmap2alm_spin(&map1, &map2, &mut alm1, &mut alm2, 2, nside, lmax, mmax, false);
    for x in &alm1{
        //println!("{:e} {:e}", x.re, x.im);
    }

    f64::alm2hmap_spin(&alm1, &alm2, &mut map1, &mut map2, 2, nside, lmax, mmax, false);
    for (&x,&y) in map1.iter().zip(map2.iter()){
        println!("{:.20} {:.20}", x, y);
    }
}
