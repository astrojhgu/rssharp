extern crate rssharp;
use std::{
    io::Write
    , fs::File
};
use rssharp::{
    transform
    , ffi::nside2npix
};

fn main() {
    let nside=512;
    let npix=nside2npix(nside);
    let lmax=nside*2-1;
    let mmax=lmax;
    let map=vec![1.0; npix];
    let alm=transform::hmap2alm_iter(&map, nside, lmax, mmax, 4);
    let mut outfile=File::create("alm.bin").unwrap();
    outfile.write(unsafe{std::slice::from_raw_parts( alm.as_ptr() as *const u8, alm.len()*std::mem::size_of::<f64>()*2)}).unwrap();
    let map1=transform::alm2hmap(&alm, nside, lmax, mmax);
    let mut outfile=File::create("map.bin").unwrap();
    outfile.write(unsafe{std::slice::from_raw_parts( map1.as_ptr() as *const u8, map1.len()*std::mem::size_of::<f64>())}).unwrap();
}
