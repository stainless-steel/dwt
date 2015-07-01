#![feature(test)]

extern crate dwt;
extern crate test;

use test::{Bencher, black_box};

#[bench] fn forward_haar_0004(bencher: &mut Bencher) { forward_haar(   4, bencher); }
#[bench] fn forward_haar_0016(bencher: &mut Bencher) { forward_haar(  16, bencher); }
#[bench] fn forward_haar_0064(bencher: &mut Bencher) { forward_haar(  64, bencher); }
#[bench] fn forward_haar_0256(bencher: &mut Bencher) { forward_haar( 256, bencher); }
#[bench] fn forward_haar_1024(bencher: &mut Bencher) { forward_haar(1024, bencher); }
#[bench] fn forward_haar_4096(bencher: &mut Bencher) { forward_haar(4096, bencher); }

fn forward_haar(size: usize, bencher: &mut Bencher) {
    let mut data = vec![42.0; size];
    let wavelet = dwt::wavelet::Haar::new();
    bencher.iter(|| black_box(dwt::forward(&mut data, &wavelet)));
}
