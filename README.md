# Kinematic coder

## Description
Kinematic Coder was supposed to be a rust project that allows users to program a robot without any coding knowledge. It was built as an assignment for my study where I got to choose a project to build in 20 days. While I was not able to finish the functionality that codes the physical robot, I still got a lot of interesting features done.

The basic idea is that an user can import their 3D models and create a object-tree with them. They can then rotate and displace the robots parts to the right place and provide some data about any potential motors in the part. For each leg, the user can then define the length and height of the step the robot should make. Given those properties, the software itself should then be able to calculate which part of the leg should rotate and how much in order for the robot to be actually walk. When given enough legs, the program should also be able to create some timing offsets for each leg so it is always stable.

Similar software exists, but they either come with an expensive robot, or limit robots to allow 6 evenly placed and rotated legs with joints all of the same length. They also require the motors to be positioned over specific axes. These are pretty large restrictions. My software is not compatible with any leg configuration, but any leg with 2 joints with different length, or 3 joints with the same length should work over any axis, as long as the actual movement is physically possible.

## Installation
After cloning the repo, you can simply run `cargo run` to run the project. This should automatically update, install and run the project.

## Usage

### Starting a project
After running the program, select the "New" button to create a new project, or open any of the existing projects.

### Uploading models
At the top-right, there is an objects tree. Click the "+ Body" button to select an ".obj" 3D model for the body of the robot. In a similar way, the user can add legs to the body. Another part of the same leg can be added by clicking the "+ Joint" button under an existing part. If any 3D model is used multiple times, the program will only keep one source for efficiency. This means that updating a 3D model, then adding the same model again will not use the new version of the model, nor will it update any existing models. The models can now be seen in the 3D scene on the left. You can drag the mouse around the scene to rotate around the robot.

### Setting model properties
In the models tree, click select any leg. This will now make this the active leg. In the bottom-right menu, several tabs of properties can be activated. Once there, checkboxes can be checked and float values can be dragged up or down to modify the properties of the selected leg.

#### Displacement
Navigate to "D" for displacement. Here you can move the leg relative to its parent to match its position on your physical robot.

![](https://raw.githubusercontent.com/SuccessfullyFailed/kinematic_coder/refs/heads/main/resource/readme_gifs/displacement.gif)

#### Motor
Navigate to "M" for motor. Here the user can click the +Motor button to start defining the motor the joint is attached to. The user can set the position relative to the joint, the axis, the rotational range and the current rotation of the motor.

![](https://raw.githubusercontent.com/SuccessfullyFailed/kinematic_coder/refs/heads/main/resource/readme_gifs/motor.gif)

#### Kinematics
For the last joint of a leg, navigate to "K" for kinematics. Here, the end-point of the leg can be positioned. Once that is set, the user can define start position, distance and height of the step that robot should take.

![](https://raw.githubusercontent.com/SuccessfullyFailed/kinematic_coder/refs/heads/main/resource/readme_gifs/kinematics.gif)

#### Controller
Navigate to "C" for controller. Here you can check the "realtime" mark for the robot to start walking. If the robot has enough legs to figure out leg timings, it will synchronize its steps too. If any joints disappear, that means the software is unable to figure out a way to move the leg at that moment. The speed and strafe values can be altered to change the speed and direction of the robot.

![](https://raw.githubusercontent.com/SuccessfullyFailed/kinematic_coder/refs/heads/main/resource/readme_gifs/controller.gif)