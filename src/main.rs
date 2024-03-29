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
    pub phi: f64,           // rotation about the Z axis by phi (heading/yaw)
    pub theta: f64,         // rotation about x' axis (rotated X axis) by theta (pitch)
    pub psi: f64,           // rotation about z' axis (rotate Z axis) by psi (roll)
    pub phi_dot: f64,
    pub theta_dot: f64,
    pub psi_dot: f64
}

impl StateVector {
    pub fn new(t_0: f64, x_0: f64, y_0: f64, z_0: f64, x_dot_0: f64, y_dot_0: f64, z_dot_0: f64, phi_0: f64, theta_0: f64, psi_0: f64, phi_dot_0: f64, theta_dot_0: f64, psi_dot_0: f64) -> Self {
        let mut new_state: StateVector = Default::default();
        new_state.t = t_0;
        new_state.x = x_0;
        new_state.y = y_0;
        new_state.z = z_0;
        new_state.x_dot = x_dot_0;
        new_state.y_dot = y_dot_0;
        new_state.z_dot = z_dot_0;
        new_state.phi = phi_0;
        new_state.theta = theta_0;
        new_state.psi = psi_0;
        new_state.phi_dot = phi_dot_0;
        new_state.theta_dot = theta_dot_0;
        new_state.psi_dot = psi_dot_0;
        return new_state
    }
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


pub fn update_2dof_turn_rate_constraint(current_state: StateVector, commanded_theta_dot: f64, dt: f64) -> StateVector {

    let omega_max: f64 = 30.0*PI/180.0;
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
    new_state.theta = commanded_theta;      // Instantaneous change. No limits imposed on heading change. Same for roll.
    new_state.theta_dot = (current_state.theta - new_state.theta)/dt;
    new_state.psi = commanded_psi;
    new_state.psi_dot = (current_state.psi - new_state.psi)/dt;

    return new_state
}

fn main() {
    let t_0: f64 = 0.0;
    let x_0:f64 = 0.0;
    let y_0:f64 = 0.0;
    let z_0:f64 = 0.0;
    let x_dot_0:f64 = 1.0;
    let y_dot_0:f64 = 1.0;
    let z_dot_0:f64 = 1.0;
    let speed = f64::sqrt(f64::powf(x_dot_0,2.0) + f64::powf(y_dot_0,2.0) + f64::powf(z_dot_0,2.0));
    let phi_0 = f64::acos(x_dot_0/speed);       // Use direction cosines to set initial orientation aligned with the velocity vector
    let theta_0 = f64::acos(y_dot_0/speed);
    let psi_0 = f64::acos(z_dot_0/speed);
    let phi_dot_0:f64 = 0.0;
    let theta_dot_0: f64 = 0.0;
    let psi_dot_0: f64 = 0.0;

    let state_0 = StateVector::new(t_0, x_0, y_0, z_0, x_dot_0, y_dot_0, z_dot_0, phi_0, theta_0, psi_0, phi_dot_0, theta_dot_0, psi_dot_0);
    println!("state_0: {}", state_0);
    let mut my_state: StateVector = Default::default();

    println!("");
    println!("update_2dof with no commanded turn");
    my_state = update_2dof(state_0, my_state.theta, 0.1);
    println!("update : {}", my_state);

    println!("");
    println!("update_2dof with commanded turn");
    let commanded_theta: f64 = 10.0*PI/180.0;
    my_state = update_2dof(state_0, commanded_theta, 0.1);
    println!("update : {}", my_state);

    println!("");
    println!("update_2dof_turn_rate_constraint with commanded turn rate constraints");
    let commanded_theta_dot: f64 = 45.0*PI/180.0; // This is 0.7853981633974483 rad, but greater than omega_max, which is 0.5235987755982988 rad
    my_state = update_2dof_turn_rate_constraint(state_0, commanded_theta_dot, 0.1);
    println!("update : {}", my_state);

    println!("");
    println!("update_3dof with commanded turn and roll");
    let commanded_psi: f64 = 10.0*PI/180.0;
    my_state = update_3dof(state_0, commanded_theta, commanded_psi, 0.1);
    println!("update : {}", my_state);
}