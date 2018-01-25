use std::{io, fmt};

struct SpinVec<T: Clone> {
    vec: Vec<T>,
}

impl<T: Clone> SpinVec<T> {
    pub fn new() -> Self {
        SpinVec {
            vec: Vec::new()
        }
    }
    
    fn push(&mut self, arg: T) {
        self.vec.push(arg);
    }
    
    fn spin(&mut self, rhs: usize) {
        let mut spun_vector: Vec<T> = vec![];
        {
            let l = self.vec.len();
            let s = l - (rhs % l);
            let (a, b) = self.vec.split_at(s);
            spun_vector.extend_from_slice(b);
            spun_vector.extend_from_slice(a);
        }
        self.vec = spun_vector;
    }
}

struct MatrixMN {
    mat: Vec<u64>,
    m: usize,
    n: usize,
    bands: Option<Vec<SpinVec<usize>>>,
}

impl MatrixMN {
    pub fn new(m: usize, n: usize) -> Self {
        MatrixMN {
            mat: vec![0; m*n],
            m,
            n,
            bands: None,
        }
    }
    
    fn gen_bands(&mut self) {
        let m = self.m;
        let n = self.n;
        let n_bands: usize = if m > n {
            if n % 2 == 0 {
                n / 2
            } else {
                n / 2 + 1
            }
        } else {
            if m % 2 == 0 {
                m / 2
            } else {
                m / 2 + 1
            }
        };
        let mut bands: Vec<SpinVec<usize>> = Vec::with_capacity(n_bands);
        for b in 0..n_bands {
            let mut band = SpinVec::new();
            for y in b..(m-b) {
                band.push((b + y * n) as usize);
            }
            for x in (1+b)..(n-1-b) {
                band.push((x + (self.m-1-b) * n) as usize);
            }
            for y in (b..(m-b)).rev() {
                band.push((n-1-b + y * n) as usize);
            }
            for x in ((1+b)..(n-1-b)).rev() {
                band.push((x + b * n) as usize);
            }
            bands.push(band);
        }
        self.bands = Some(bands);
    }
    
    fn rotate(&mut self, times: usize) -> Result<(), ()> {
        if let Some(ref mut bands) = self.bands {
            for band in bands {
                band.spin(times);
            }
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn apply_rotation(&mut self) -> Result<(), ()> {
        if let Some(ref bands) = self.bands {
            let mut new_mat = vec![0; self.m*self.n];
            for (b, band) in bands.iter().enumerate() {
                let mut i = 0;
                for y in b..(self.m-b) {
                    new_mat[b + y*self.n] = self.mat[band.vec[i]];
                    i+=1;
                }
                for x in (1+b)..(self.n-1-b) {
                    new_mat[x + (self.m-1-b) * self.n] = self.mat[band.vec[i]];
                    i+=1;
                }
                for y in (b..(self.m-b)).rev() {
                    new_mat[self.n-1-b + y*self.n] = self.mat[band.vec[i]];
                    i+=1;
                }
                for x in ((1+b)..(self.n-1-b)).rev() {
                    new_mat[x + b*self.n] = self.mat[band.vec[i]];
                    i+=1;
                }
            }
            self.mat = new_mat;
            Ok(())
        } else {
            Err(())
        }
        
    }
}

impl fmt::Display for MatrixMN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rowi in 0..self.m {
            for coli in 0..self.n {
                write!(f, "{} ", self.mat[coli + rowi * self.n])?;
            }
            write!(f, "\n")?;
        }
        
        Ok(())
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut params: [usize; 3] = [0; 3];
    for (i, ns) in buf.split(' ').enumerate() {
        params[i] = ns.trim_right().parse::<usize>().unwrap();
    }
    let mut mat = MatrixMN::new(params[0], params[1]);
    
    for row in 0..mat.m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for (col, ns) in buf.split(' ').enumerate() {
            mat.mat[col + row * mat.n] = ns.trim().parse::<u64>().unwrap();
        }
    }
    mat.gen_bands();
    mat.rotate(params[2]).unwrap();
    mat.apply_rotation().unwrap();
    println!("{}", mat);
}
