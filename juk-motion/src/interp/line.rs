//! 3D line interpolator based on the Bresenham's algorithm

use crate::interp::Step;

#[derive(defmt::Format)]
enum Axis {
    X,
    Y,
    Z,
}

/// Line interpolator working in relative coordinates only.
#[derive(defmt::Format)]
pub struct LineGenerator {
    lx: Step,
    ly: Step,
    lz: Step,
    ax: u32,
    ay: u32,
    az: u32,
    err_x: u32,
    err_y: u32,
    err_z: u32,
    major: Axis,
    len: usize,
    i: usize,
}

impl LineGenerator {
    /// Construct a [`LineGenerator`] from the final position relative to the current one (in steps).
    pub const fn new(dx: i32, dy: i32, dz: i32) -> Self {
        let lx = Step::from_displacement(dx);
        let ly = Step::from_displacement(dy);
        let lz = Step::from_displacement(dz);

        let ax = dx.unsigned_abs();
        let ay = dy.unsigned_abs();
        let az = dz.unsigned_abs();

        let (major, len) = if ax >= ay && ax >= az {
            (Axis::X, ax)
        } else if ay >= ax && ay >= az {
            (Axis::Y, ay)
        } else {
            (Axis::Z, az)
        };

        Self {
            lx,
            ly,
            lz,
            ax,
            ay,
            az,
            err_x: 0,
            err_y: 0,
            err_z: 0,
            major,
            len: len as usize,
            i: 0,
        }
    }

    /// Return the major axis' length of the line in steps.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return an iterator over actions to take in order to perform the movement.
    ///
    /// The tuple is in the order: `(x, y, z)`. See [`Step`] for details about the action.
    pub fn step_iter(self) -> LineIter {
        LineIter(self)
    }

    fn next_step(&mut self) -> Option<(Step, Step, Step)> {
        if self.i >= self.len {
            return None;
        }

        let mut move_x = Step::None;
        let mut move_y = Step::None;
        let mut move_z = Step::None;

        match self.major {
            Axis::X => {
                move_x = self.lx;

                self.err_y += self.ay;
                self.err_z += self.az;

                if self.err_y >= self.ax {
                    move_y = self.ly;
                    self.err_y -= self.ax;
                }

                if self.err_z >= self.ax {
                    move_z = self.lz;
                    self.err_z -= self.ax;
                }
            }
            Axis::Y => {
                move_y = self.ly;

                self.err_x += self.ax;
                self.err_z += self.az;

                if self.err_x >= self.ay {
                    move_x = self.lx;
                    self.err_x -= self.ay;
                }

                if self.err_z >= self.ay {
                    move_z = self.lz;
                    self.err_z -= self.ay;
                }
            }
            Axis::Z => {
                move_z = self.lz;

                self.err_x += self.ax;
                self.err_y += self.ay;

                if self.err_x >= self.az {
                    move_x = self.lx;
                    self.err_x -= self.az;
                }

                if self.err_y >= self.az {
                    move_y = self.ly;
                    self.err_y -= self.az;
                }
            }
        }

        self.i += 1;
        Some((move_x, move_y, move_z))
    }
}

/// Iterator over [`LineGenerator`]'s step actions.
pub struct LineIter(LineGenerator);

impl Iterator for LineIter {
    type Item = (Step, Step, Step);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_step()
    }
}

impl ExactSizeIterator for LineIter {
    fn len(&self) -> usize {
        self.0.len()
    }
}
