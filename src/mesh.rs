use crate::error::Error;
use std::fs;
use std::io::Write;

pub struct PointSource {
    x: usize,
    y: usize,
    peak_time: f64,
    pulse_width: f64,
}

pub struct Mesh {
    w: usize,
    h: usize,

    dx: f64,
    dy: f64,
    dt: f64,

    ez: Vec<f64>,
    hx: Vec<f64>,
    hy: Vec<f64>,

    time: f64,

    sources: Vec<PointSource>,
}

impl Mesh {
    fn index(x: usize, y: usize, w: usize) -> usize {
        y * w + x
    }

    pub fn step(&mut self) {
        let chx = self.dt / self.dy;
        let chy = self.dt / self.dx;
        let cez = self.dt / self.dx;

        // Update H_x
        for x in 0..self.w {
            for y in 0..self.h - 1 {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x, y + 1, self.w);

                self.hx[i0] -= (self.ez[i1] - self.ez[i0]) * chx;
            }
        }

        // Update H_y
        for x in 0..self.w - 1 {
            for y in 0..self.h {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x + 1, y, self.w);

                self.hy[i0] += (self.ez[i1] - self.ez[i0]) * chy;
            }
        }

        // Update E_z
        for x in 1..self.w - 1 {
            for y in 1..self.h - 1 {
                let i0 = Self::index(x, y, self.w);
                let i1 = Self::index(x - 1, y, self.w);
                let i2 = Self::index(x, y - 1, self.w);

                self.ez[i0] += ((self.hy[i0] - self.hy[i1]) - (self.hx[i0] - self.hx[i2])) * cez;
            }
        }

        for s in &self.sources {
            let i = Self::index(s.x, s.y, self.w);
            let peak_time = s.peak_time * self.dt;
            let pulse_width = s.pulse_width * self.dt;
            self.ez[i] += (-(self.time - peak_time).powf(2.) / pulse_width.powf(2.)).exp();
        }

        self.time += self.dt;
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

pub struct MeshBuilder {
    w: usize,
    h: usize,
    dx: f64,
    dy: f64,
    safety_factor: f64,
    sources: Vec<PointSource>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            w: 500,
            h: 500,
            dx: 1.0,
            dy: 1.0,
            safety_factor: 0.9,
            sources: Vec::new(),
        }
    }

    pub fn with_width(mut self, w: usize) -> Self {
        self.w = w;
        self
    }

    pub fn with_height(mut self, h: usize) -> Self {
        self.h = h;
        self
    }

    pub fn with_dx(mut self, dx: f64) -> Self {
        self.dx = dx;
        self
    }

    pub fn with_dy(mut self, dy: f64) -> Self {
        self.dy = dy;
        self
    }

    pub fn with_safety_factor(mut self, safety_factor: f64) -> Self {
        self.safety_factor = safety_factor;
        self
    }

    pub fn with_point_source(
        mut self,
        x: usize,
        y: usize,
        peak_time: f64,
        pulse_width: f64,
    ) -> Self {
        self.sources.push(PointSource {
            x,
            y,
            peak_time,
            pulse_width,
        });
        self
    }

    pub fn build(self) -> Mesh {
        const C: f64 = 1.00;

        let dt: f64 = self.safety_factor * self.dx / (C * 2f64.sqrt()); // consider CFL condition in futur

        Mesh {
            w: self.w,
            h: self.h,
            dx: self.dx,
            dy: self.dy,
            dt,
            ez: vec![0.; self.w * self.h],
            hx: vec![0.; self.w * self.h],
            hy: vec![0.; self.w * self.h],
            time: 0.,
            sources: self.sources,
        }
    }
}
