# Elea - Quadcopter physics sim

Exploring physics and drone simulation!


## TODOs

### Foundation
- Position
- Velocity
- Acceleration
- Rotation matricesor quaternions for orientation (Yaw, Pitch, Roll)
- Time stepping integration (Euler Method)
- Angular Velocity

### Rigid body Dynamics
*WE ARE HERE!!*
- Mass
- Center of Mass
- Moment of intertia
- Linear and Angular Momentum
- Forces
- Torques

By the end of this, we have a basic physics object we can move and rotate in 3d space.

### Drone-Specific Physics
Model rotors (as thrust generators).
Thrust magnitude depends on rotor speed (RPM squared relationship). Each rotor creates both upward thrust and reactive torque. Position the rotors correctly relative to the drone's center of mass.


### Flight Control
- Rotor speed changes affect motion
- Collective thrust (for altitude)
- Differential Thrust for roll/pitch/yaw
- PID Controllers to stabilize drone and follow simple commands

### Enviromental Forces
- Gravity
- Air Resistance (Drag proportional to velocity squared)
- Ground collision detection and response

### Advanced Dynamics
- Gyroscopic effects from spinning rotors
- Rotor blade flapping
- Motor Dynamics (Rotors dont instantly change speed)
- Battery Life/Drain affects on power/power consumption

### Weather
- Begin with steady wind as constant force vector
- Turbelence using noise functions
- Gusts and wind shear
- Thermals
- Downdrafts
- Air density changes with alt. and temp. (how does this affect thrust?)

### Sensor simulation
  - Simulated Inertial Measurement Unit (IMU) with:
    - Noise
    - GPS (w/ accuracy variations)
    - Barometric pressure sensors
