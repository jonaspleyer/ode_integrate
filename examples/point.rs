// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Mul,Add,AddAssign};

use ode_solvers::concepts::errors::CalcError;
use ode_solvers::solvers::fixed_step::{Euler};
use ode_solvers::concepts::steppers::Stepper;


fn rhs(y: &Point, dy: &mut Point, _t: &f64, p: &[f64; 2]) -> Result<(), CalcError> {
    dy.x = -p[0] * y.x;
    dy.y = -p[0] * y.y;
    Ok(())
}


#[derive(Debug,Copy,Clone,PartialEq)]
struct Point {
    x: f64,
    y: f64
}


impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Mul<f64> for Point {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self {
        Point{ x: self.x * rhs, y: self.y * rhs }
    }
}


impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point{ x: self * rhs.x, y: self * rhs.y }
    }
}


fn main() {
    let mut y  = Point{x: 1.0, y: 2.0};
    let mut dy = Point{x: 1.0, y: 2.0};

    let p = [2.0, 0.2];

    let t0 = 0.0;
    let tend = 3.0;
    let dt = 0.05;
    let mut t  = t0;

    let eu = Euler {};

    while t<tend {
        eu.do_step_add(&rhs, &mut y, &mut dy, &t, &dt, &p).unwrap();
        t += dt;
        println!("t={:6.4} p=[{:6.4} {:6.4}]", t, y.x, y.y);
    }
}