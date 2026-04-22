//! Arc interpolator using a rotation matrix

use core::iter::FlatMap;

use crate::{
    Error,
    interp::{ArcDir, LineGenerator, Step, line::LineIter},
};

/// 2D arc interpolator with support for helix movements.
#[derive(defmt::Format, Clone, Copy)]
pub struct ArcGenerator {
    // arc center
    cx: f32,
    cy: f32,
    // vector for rotations
    vx: f32,
    vy: f32,
    // rotation trig
    sin_step: f32,
    cos_step: f32,
    // helix support
    z: f32,
    dz: f32,
    // last position
    last_x: i32,
    last_y: i32,
    last_z: i32,
    // remaining segments
    remaining: usize,
}

impl ArcGenerator {
    /// Allowed deviation from the actual arc in steps. (Lower is more accurate)
    const ARC_ERROR: f32 = 0.05;

    /// Construct an [`ArcGenerator`].
    ///
    /// # Arguments
    ///
    /// - `dx`, `dy`: relative displacement from the current position (arc endpoint),
    /// - `dz`: Z displacement for helical movement,
    /// - `r`: radius of the arc,
    /// - `dir`: direction of the arc (see [`ArcDir`]).
    ///
    /// # Errors
    ///
    /// - `r = 0`: no radius means no arc,
    /// - zero displacement: no movement,
    /// - radius too small: can't draw an arc like that.
    pub fn new(dx: i32, dy: i32, dz: i32, r: u32, dir: ArcDir) -> Result<Self, Error> {
        if r == 0 {
            return Err(Error::ZeroRadius);
        }

        let r = r as f32;
        let dx = dx as f32;
        let dy = dy as f32;
        let dz = dz as f32;

        let d = libm::hypotf(dx, dy);
        if d == 0.0 {
            return Err(Error::ZeroDisplacement);
        } else if d > 2.0 * r {
            return Err(Error::ImpossibleGeometry);
        }

        // midpoint
        let mx = 0.5 * dx;
        let my = 0.5 * dy;

        // distance from midpoint to center
        let h = libm::sqrtf(f32::max((r * r) - (0.25 * d * d), 0.0));
        // perpendicular
        let ux = -dy / d;
        let uy = dx / d;

        // center coordinates
        let (cx, cy) = match dir {
            ArcDir::Neg => (mx - ux * h, my - uy * h),
            ArcDir::Pos => (mx + ux * h, my + uy * h),
        };

        // vector from center to start and end
        let v0x = -cx;
        let v0y = -cy;
        let v1x = dx - cx;
        let v1y = dy - cy;

        // angles
        let a0 = libm::atan2f(v0y, v0x);
        let a1 = libm::atan2f(v1y, v1x);
        let mut delta = a1 - a0;

        // normalize to (-pi, pi]
        while delta <= -core::f32::consts::PI {
            delta += 2.0 * core::f32::consts::PI;
        }
        while delta > core::f32::consts::PI {
            delta -= 2.0 * core::f32::consts::PI;
        }

        let delta = match dir {
            ArcDir::Pos => {
                if delta < 0.0 {
                    delta + 2.0 * core::f32::consts::PI
                } else {
                    delta
                }
            }
            ArcDir::Neg => {
                if delta > 0.0 {
                    delta - 2.0 * core::f32::consts::PI
                } else {
                    delta
                }
            }
        };

        let sweep = libm::fabsf(delta);

        let cap = 1.0 - (Self::ARC_ERROR / r);
        let theta_step = if cap <= -1.0 || cap >= 1.0 {
            2.0 * core::f32::consts::PI
        } else {
            2.0 * libm::acosf(cap)
        };

        let n = libm::ceilf(sweep / theta_step) as usize;

        let angle_step = delta / (n as f32);

        let cos_step = libm::cosf(angle_step);
        let sin_step = libm::sinf(angle_step);

        // helix support
        let dz = dz / (n as f32);

        Ok(Self {
            cx,
            cy,
            vx: v0x,
            vy: v0y,
            sin_step,
            cos_step,
            z: 0.0,
            dz,
            last_x: 0,
            last_y: 0,
            last_z: 0,
            remaining: n,
        })
    }

    /// Calculate how many step actions to take to draw the arc.
    pub fn len(&self) -> usize {
        let s = self.clone();

        // instead of constructing a LineGenerator, calculate the major axes and accumulate the
        // result
        SegmentIter(s).fold(0, |len, (dx, dy, dz)| {
            len + dx
                .unsigned_abs()
                .max(dy.unsigned_abs())
                .max(dz.unsigned_abs()) as usize
        })
    }

    /// Return an iterator over actions to take in order to draw the arc.
    ///
    /// The tuple is in the order: `(x, y, z)`. See [`Step`] for details about the action.
    pub fn step_iter(self) -> ArcIter<impl FnMut((i32, i32, i32)) -> LineIter> {
        ArcIter {
            iter: SegmentIter(self)
                .flat_map(|(dx, dy, dz)| LineGenerator::new_unchecked(dx, dy, dz).step_iter()),
            len: self.len(),
        }
    }

    fn next_line(&mut self) -> Option<(i32, i32, i32)> {
        if self.remaining == 0 {
            return None;
        }

        // rotate vector
        let vx = self.cos_step * self.vx - self.sin_step * self.vy;
        let vy = self.sin_step * self.vx + self.cos_step * self.vy;

        // absolute position of the point
        let abs_x = libm::roundf(self.cx + vx) as i32;
        let abs_y = libm::roundf(self.cy + vy) as i32;

        // helix
        self.z += self.dz;
        let abs_z = libm::roundf(self.z) as i32;

        // deltas from previous position
        let dx = abs_x - self.last_x;
        let dy = abs_y - self.last_y;
        let dz = abs_z - self.last_z;

        self.vx = vx;
        self.vy = vy;
        self.last_x = abs_x;
        self.last_y = abs_y;
        self.last_z = abs_z;
        self.remaining -= 1;

        Some((dx, dy, dz))
    }
}

struct SegmentIter(ArcGenerator);

impl Iterator for SegmentIter {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_line()
    }
}

/// Iterator over [`ArcGenerator`]'s step actions.
pub struct ArcIter<F>
where
    F: FnMut((i32, i32, i32)) -> LineIter,
{
    iter: FlatMap<SegmentIter, LineIter, F>,
    len: usize,
}

impl<F> Iterator for ArcIter<F>
where
    F: FnMut((i32, i32, i32)) -> LineIter,
{
    type Item = (Step, Step, Step);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().inspect(|_| self.len -= 1)
    }
}

impl<F> ExactSizeIterator for ArcIter<F>
where
    F: FnMut((i32, i32, i32)) -> LineIter,
{
    fn len(&self) -> usize {
        self.len
    }
}
