use std::fmt;
use std::f64::consts::PI;

#[derive(Debug, Default, Copy, Clone)]
pub struct StateVector 
{
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dot: f64,
    pub y_dot: f64,
    pub z_dot: f64,
    pub phi: f64,
    pub theta: f64,
    pub psi: f64,
    pub phi_dot: f64,
    pub theta_dot: f64,
    pub psi_dot: f64
}

impl fmt::Display for StateVector 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} t, {} x, {} y, {} z, {} x_dot, {} y_dot, {} z_dot, {} phi, {} theta, {} psi, {} phi_dot, {} theta_dot, {} psi_dot)", 
                self.t, self.x, self.y, self.z, self.x_dot, self.y_dot, self.z_dot, self.phi, self.theta, self.psi, self.phi_dot, self.theta_dot, self.psi_dot)
    }
}

pub fn update_2dof(current_state: StateVector, commanded_theta: f64, dt: f64) -> StateVector {

    let mut new_state = current_state;
    let speed = f64::sqrt(f64::powf(current_state.x_dot,2.0) + f64::powf(current_state.y_dot,2.0));
    new_state.x_dot = speed*f64::cos(current_state.theta);
    new_state.y_dot = speed*f64::sin(current_state.theta);
    new_state.x += new_state.x_dot*dt;
    new_state.y += new_state.y_dot*dt;
    new_state.t += dt;
    new_state.theta = commanded_theta;
    new_state.theta_dot = (current_state.theta - new_state.theta)/dt;

    return new_state
}

pub fn update_3dof(current_state: StateVector, commanded_theta: f64, commanded_psi: f64, dt: f64) -> StateVector {

    let mut new_state = current_state;
    let speed = f64::sqrt(f64::powf(current_state.x_dot,2.0) + f64::powf(current_state.y_dot,2.0) + f64::powf(current_state.z_dot,2.0));
    new_state.x_dot = speed*f64::cos(current_state.theta)*f64::cos(current_state.psi);
    new_state.y_dot = speed*f64::sin(current_state.theta)*f64::sin(current_state.psi);
    new_state.z_dot = speed*f64::sin(current_state.theta);
    new_state.x += new_state.x_dot*dt;
    new_state.y += new_state.y_dot*dt;
    new_state.z += new_state.z_dot*dt;
    new_state.t += dt;
    new_state.theta = commanded_theta;
    new_state.psi = commanded_psi;

    return new_state
}

pub fn update_2dof_turn_rate_constraint(current_state: StateVector, commanded_theta_dot: f64, dt: f64) -> StateVector {

    let omega_max: f64 = 10.0;
    let mut new_state = current_state;
    let speed = f64::sqrt(f64::powf(current_state.x_dot,2.0) + f64::powf(current_state.y_dot,2.0));
    new_state.x_dot = speed*f64::cos(current_state.theta);
    new_state.y_dot = speed*f64::sin(current_state.theta);
    new_state.x += new_state.x_dot*dt;
    new_state.y += new_state.y_dot*dt;
    if commanded_theta_dot > omega_max {
        new_state.theta_dot = omega_max;
    } 
    else {
        new_state.theta_dot = commanded_theta_dot;
    }
    new_state.theta += new_state.theta_dot*dt;
    new_state.t += dt; 

    return new_state
}

fn main() {
    let mut my_state: StateVector = Default::default();
    my_state.x_dot = 1.0;
    my_state.y_dot = 1.0;
    my_state.theta = f64::atan2(my_state.x_dot, my_state.y_dot);

    println!("update_2dof with no commanded turn");
    println!("initial: {}", my_state);
    my_state = update_2dof(my_state, my_state.theta, 0.1);
    println!("update : {}", my_state);

    println!("");
    println!("update_2dof with commanded turn");
    println!("initial: {}", my_state);
    let commanded_theta: f64 = 10.0*PI/180.0;
    my_state = update_2dof(my_state, commanded_theta, 0.1);
    println!("update : {}", my_state);

}