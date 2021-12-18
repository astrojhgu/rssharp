#include <stdio.h>
#include <stdlib.h>
#include "include/sharp.h"
#include <vector>
#include <complex>
#include <fstream>
using namespace std;
int main()
{
    using FTYPE=double;
    int nside=512;
    std::vector<FTYPE> data(12*nside*nside);
    for(int i=0;i<12*nside*nside;++i){
        data[i]=1.0;
    }
    int lmax=nside*2-1;
    int mmax=nside*2-1;
    int nalm=n_alm(lmax, mmax);
    std::vector<std::complex<FTYPE>> alm(nalm);

    hmap2alm(data.data(), alm.data(), nside, lmax, mmax, 0);
    ofstream ofs1("alm.dat");
    ofs1.write((char*)alm.data(), sizeof(std::complex<FTYPE>)*alm.size());
    alm2hmap(alm.data(), data.data(), nside, lmax, mmax, 0);
    ofstream ofs2("map.dat");
    ofs2.write((char*)data.data(), sizeof(FTYPE)*data.size());
}