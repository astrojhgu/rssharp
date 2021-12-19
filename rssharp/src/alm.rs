use std::{
    default::Default
    , ops::{
        Index
        , IndexMut
    }
};

pub fn n_alm(lmax:usize, mmax:usize)->usize
{ 
  ((mmax+1)*(mmax+2))/2 + (mmax+1)*(lmax-mmax)
}



#[derive(Clone)]
pub struct AlmBase{
    pub lmax: usize
    , pub mmax: usize
    , tval: usize
}

impl AlmBase{
    pub fn new(lmax: usize, mmax: usize)->Self{
        let tval=2*lmax+1;
        Self{
            lmax, mmax, tval
        }
    }

    pub fn index_l0(&self, m: usize)->usize{
        (m*(self.tval-m))>>1
    }

    pub fn index(&self, l: usize, m: usize)->usize{
        self.index_l0(m)+l
    }
}
#[derive(Clone)]
pub struct Alm<T>{
    base: AlmBase,
    pub data: Vec<T>
}

impl<T> Alm<T>
where T:Default+Clone{
    pub fn new(lmax:usize, mmax: usize, data: Vec<T>)->Self{
        assert_eq!(data.len(), n_alm(lmax, mmax));
        Self{
            base: AlmBase::new(lmax, mmax)
            , data
        }
    }

    pub fn zeros(lmax: usize, mmax: usize)->Self{
        let data=vec![T::default(); n_alm(lmax, mmax)];
        Self::new(lmax, mmax, data)
    }

    pub fn lmax(&self)->usize{
        self.base.lmax
    }

    pub fn mmax(&self)->usize{
        self.base.mmax
    }
}

impl<T> Index<(usize, usize)> for Alm<T>{
    type Output=T;
    fn index(&self, (l,m): (usize, usize))->&T{
        //&self.data[self.base.index(l, m)]
        self.data.index(self.base.index(l, m))
    }
}

impl<T> IndexMut<(usize, usize)> for Alm<T>{
    fn index_mut(&mut self, (l,m): (usize, usize))->&mut T{
        self.data.index_mut(self.base.index(l,m))
    }
}

