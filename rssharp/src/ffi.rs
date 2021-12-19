use num::{
    complex::{
        Complex
    }
};

pub fn n_alm(lmax: usize, mmax:usize)->usize{
    unsafe{rssharp_sys::n_alm(lmax, mmax)}
}

pub trait SharpFloat:Sized{
    fn alm2hmap(alm:&[Complex<Self>], map:&mut [Self], nside: usize, lmax: usize, mmax: usize, add:bool);
    fn alm2hmap_spin(alm1: &[Complex<Self>], alm2: &[Complex<Self>], map1: &mut [Self], map2: &mut [Self], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool);
    fn hmap2alm(map:&[Self], alm:&mut [Complex<Self>], nside: usize, lmax: usize, mmax: usize, add:bool);
    fn hmap2alm_spin(map1: &[Self], map2: &[Self], alm1: &mut [Complex<Self>], alm2: &mut [Complex<Self>], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool);
}

pub fn nside2npix(nside: usize)->usize{
    12*nside.pow(2)
}

impl SharpFloat for f64{
    fn alm2hmap(alm:&[Complex<Self>], map:&mut [Self], nside: usize, lmax: usize, mmax: usize, add:bool){
        assert_eq!(alm.len(), n_alm(lmax, mmax));
        assert_eq!(map.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::alm2hmap(alm.as_ptr(), map.as_mut_ptr(), nside, lmax, mmax, if add {1} else {0})
        }
    }
    fn alm2hmap_spin(alm1: &[Complex<Self>], alm2: &[Complex<Self>], map1: &mut [Self], map2: &mut [Self], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool){
        assert_eq!(alm1.len(), alm2.len());
        assert_eq!(map1.len(), map2.len());
        assert_eq!(alm1.len(), n_alm(lmax, mmax));
        assert_eq!(map1.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::alm2hmap_spin(alm1.as_ptr(), alm2.as_ptr(), map1.as_mut_ptr(), map2.as_mut_ptr(), spin, nside, lmax, mmax, if add {1} else{0})
        }
    }
    fn hmap2alm(map:&[Self], alm:&mut [Complex<Self>], nside: usize, lmax: usize, mmax: usize, add:bool){
        assert_eq!(alm.len(), n_alm(lmax, mmax));
        assert_eq!(map.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::hmap2alm(map.as_ptr(), alm.as_mut_ptr(), nside, lmax, mmax, if add {1} else {0})
        }
    }
    fn hmap2alm_spin(map1: &[Self], map2: &[Self], alm1: &mut [Complex<Self>], alm2: &mut [Complex<Self>], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool){
        assert_eq!(alm1.len(), alm2.len());
        assert_eq!(map1.len(), map2.len());
        assert_eq!(alm1.len(), n_alm(lmax, mmax));
        assert_eq!(map1.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::hmap2alm_spin(map1.as_ptr(), map2.as_ptr(), alm1.as_mut_ptr(), alm2.as_mut_ptr(), spin, nside, lmax, mmax, if add {1} else {0})
        }       
    }
}

impl SharpFloat for f32{
    fn alm2hmap(alm:&[Complex<Self>], map:&mut [Self], nside: usize, lmax: usize, mmax: usize, add:bool){
        assert_eq!(alm.len(), n_alm(lmax, mmax));
        assert_eq!(map.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::alm2hmap_f(alm.as_ptr(), map.as_mut_ptr(), nside, lmax, mmax, if add {1} else {0})
        }
    }
    fn alm2hmap_spin(alm1: &[Complex<Self>], alm2: &[Complex<Self>], map1: &mut [Self], map2: &mut [Self], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool){
        assert_eq!(alm1.len(), alm2.len());
        assert_eq!(map1.len(), map2.len());
        assert_eq!(alm1.len(), n_alm(lmax, mmax));
        assert_eq!(map1.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::alm2hmap_spin_f(alm1.as_ptr(), alm2.as_ptr(), map1.as_mut_ptr(), map2.as_mut_ptr(), spin, nside, lmax, mmax, if add {1} else{0})
        }
    }
    fn hmap2alm(map:&[Self], alm:&mut [Complex<Self>], nside: usize, lmax: usize, mmax: usize, add:bool){
        assert_eq!(alm.len(), n_alm(lmax, mmax));
        assert_eq!(map.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::hmap2alm_f(map.as_ptr(), alm.as_mut_ptr(), nside, lmax, mmax, if add {1} else {0})
        }
    }
    fn hmap2alm_spin(map1: &[Self], map2: &[Self], alm1: &mut [Complex<Self>], alm2: &mut [Complex<Self>], spin: i32, nside: usize, lmax: usize, mmax: usize, add: bool){
        assert_eq!(alm1.len(), alm2.len());
        assert_eq!(map1.len(), map2.len());
        assert_eq!(alm1.len(), n_alm(lmax, mmax));
        assert_eq!(map1.len(), nside2npix(nside));
        unsafe{
            rssharp_sys::hmap2alm_spin_f(map1.as_ptr(), map2.as_ptr(), alm1.as_mut_ptr(), alm2.as_mut_ptr(), spin, nside, lmax, mmax, if add {1} else {0})
        }       
    }
}
