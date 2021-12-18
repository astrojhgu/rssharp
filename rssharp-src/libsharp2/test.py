#!/usr/bin/env python3
import numpy as np
import pysharp
nside=512
lmax=nside*2-1
mmax=lmax
job=pysharp.sharpjob_d()
job.set_triangular_alm_info(lmax,mmax) 
job.set_Healpix_geometry(nside)

data=np.fromfile('map.dat', dtype=np.float64)
data1=job.alm2map(job.map2alm(np.ones(12*nside**2)))
print((data==data1).all())
print(data)
print(data1)

