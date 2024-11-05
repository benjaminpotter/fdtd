use std::fs;
use std::io::{Error, Write};

pub struct Mesh {
    w: usize,
    h: usize,

    ez: Vec<f64>,
    hx: Vec<f64>,
    hy: Vec<f64>,

    time: f64,
}

impl Mesh {
    pub fn new() -> Self {
        let width: usize = 500;
        let height: usize = 500;
        Mesh {
            w: width,
            h: height,
            ez: vec![0.; width * height],
            hx: vec![0.; width * height],
            hy: vec![0.; width * height],
            time: 0.,
        }
    }

    fn index(x: usize, y: usize, w: usize) -> usize {
        y * w + x
    }

    pub fn step(&mut self) {
        // Update H_x
        for x in 0..self.w {
            for y in 0..self.h - 1 {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x, y + 1, self.w);

                self.hx[i0] -= (self.ez[i1] - self.ez[i0]) / 377.;
            }
        }

        // Update H_y
        for x in 0..self.w - 1 {
            for y in 0..self.h {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x + 1, y, self.w);

                self.hy[i0] += (self.ez[i1] - self.ez[i0]) / 377.;
            }
        }

        // Update E_z
        for x in 1..self.w - 1 {
            for y in 1..self.h - 1 {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x - 1, y, self.w);
                let i2 = Self::index(x, y - 1, self.w);

                self.ez[i0] += ((self.hy[i0] - self.hy[i1]) - (self.hx[i0] - self.hx[i2])) * 377.;
            }
        }

        let mid = Self::index(self.w / 2, self.h / 2, self.w);
        self.ez[mid] += (-(self.time - 20.0).powf(2.) / 100.0).exp();

        self.time += 1.;
    }

    pub fn serialize(&self, fd: &mut fs::File) -> Result<(), Error> {
        for j in 0..self.h {
            for i in 0..self.w {
                fd.write(format!["{:10.2} ", self.ez[j * self.w + i].abs()].as_bytes())?;
            }

            fd.write(b"\n")?;
        }

        fd.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
