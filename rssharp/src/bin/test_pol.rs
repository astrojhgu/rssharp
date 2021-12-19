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

    let map_t=vec![1.0_f64; npix];
    let map_q:Vec<_>=(0..npix).map(|i| (i%2) as i32 as f64*0.1).collect();
    let map_u:Vec<_>=(0..npix).map(|i| ((i+1)%2) as i32 as f64*0.2).collect();

    let (alm_t, alm_g, alm_c)=transform::hmap2alm_pol_iter(&map_t, &map_q, &map_u, nside, lmax, mmax, 20);
    let (map_t2, map_q2, map_u2)=transform::alm2hmap_pol(&alm_t, &alm_g, &alm_c, nside, lmax, mmax);
    for (&t,(&q,&u)) in map_t2.iter().zip(map_q2.iter().zip(map_u2.iter())){
        println!("{:.20} {:.20} {:.20}", t,q,u);
    }
}
