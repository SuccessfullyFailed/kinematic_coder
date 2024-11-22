/*
	I made the decision to, for every modification operation of a vertex, keep all parameters borrowed.
	As a vertex by itself is not very exciting, it is easy to assume the same operation will be done for multiple vertices at once.
	This way, large operations can save some time by not having to copy all parameters for each vertex, even for small operations.
	Because of the large operation count, the rotations in this file will also be in radians opposed to degrees to not have to convert a lot.
*/


pub type Vertex = [f32; 3];

impl VertexMath for Vertex {

	/// Get a reference to the position of the struct.
	fn get_position(&self) -> &Vertex { self }
	
	/// Get a mutable reference to the position of the struct.
	fn get_position_mut(&mut self) -> &mut Vertex { self }
}

pub trait VertexMath {

	/* POSITION GETTER METHODS */

	/// Get a reference to the position of the struct.
	fn get_position(&self) -> &Vertex;

	/// Get a mutable reference to the position of the struct.
	fn get_position_mut(&mut self) -> &mut Vertex;

	/// Get a literal position.
	fn clone_position(&self) -> Vertex{
		let current_position:&Vertex = self.get_position();
		[current_position[0], current_position[1], current_position[2]]
	}



	/* DISPLACEMENT METHODS */

	/// Displace the vertex in-place.
	fn displace(&mut self, offset:&Vertex) {
		let pos:&mut Vertex = self.get_position_mut();
		for axis in 0..3 {
			pos[axis] += offset[axis];
		}
	}

	/// Return a displaced version of the vertex.
	fn displaced(&self, offset:&Vertex) -> Vertex {
		let current_position:&Vertex = self.get_position();
		[current_position[0] + offset[0], current_position[1] + offset[1], current_position[2] + offset[2]]
	}

	/// Get the opposite of the vertex.
	fn negative(&self) -> Vertex {
		let current_position:&Vertex = self.get_position();
		[-current_position[0], -current_position[1], -current_position[2]]
	}



	/* ROTATION METHODS */

	/// Rotate this vertex in-place.
	/// TODO: Animation is outdated and should be updated.
	fn rotate(&mut self, rotation:&[f32; 3], anchor:&Option<[f32; 3]>) {

		// General idea behind the method:
		// 	In my first rotation method attempt, there were some problems with the 3-axis rotations.
		// 	Because i had to calculate one axis of rotation after another, one of the rotations could have an effect on another axis.
		// 	This would cause the rotation around that axis to be inaccurate, and would stretch and warp meshes built from vertices.
		// 	
		// 	In order to avoid this, this new version of the method will instead calculate an 3-dimensional axis the source location can rotate around.
		// 	This means the vertex will rotate around 3 global axes, but will do so in one virtual local axis.
		// 	The axis will still need to be calculated in 3 axes, but as this axis does not modify throughout the calculation of it, it will not warp in the algorithm.
		// 	Due to the complexity of the calculation, i have added a lot of unnecessary spaces and brackets to help keep the code tidy and understandable.
		// 	If you have a problem with unnecessary spaces or brackets, feel free to collapse the method in your IDE.
		
		// Calculating a rotation of a coordinate in a 2D graph:
		// 	Before starting to understand how the rotation through 3D space over a virtual axis works, it is crucial to understand how a rotation through 2D space works.
		// 	
		// 	To calculate the effect on the X axis of a rotation around a 2D axis, `new_x = x * rotation_x.cos() + y * rotation_y.sin()` is used.
		// 	If you rotate [point.x, 0.0] around the 0.0 point of a 2D graph, `x * rotation_x.cos()` explains the ratio of the new points X value compared to the Y value.
		// 	Cos always needs an angle, adjacent side and hypotenuse, but if we imagine this as a scale, we can simply use '1' as the hypotenuse and leave the hypotenuse out of the equation.
		// 	In that case, Cos(angle) equals the adjacent side, and in the case of no hypotenuse, the 'scale' of the points X value.
		// 	This scale can be applied to the X value of the point to get the effect on the X axis the rotation will have.
		// 	We can then do a similar thing for `y * rotation_y.sin()`, except this shows how much the rotation of [0.0, point.y] will have on the X value of the new point.
		// 	
		// 	To calculate the effect on the Y axis of a rotation around a 2D axis, a very similar thing can be done by using `new_y = x * rotation_x.sin() - y * rotation_y.cos()`.
		// 	This works exactly the same way, except it calculates the effect on the other axis.
		// 	It is important to subtract `y * rotation_y.cos()` of the equation because the rotations are applied counter-clockwise and moving upwards from the X-axis results in a negative effect on the Y value.
		
		use std::f32::consts::PI;

		// Prepare the mainly used variables for easier use in the larger calculations.
		let double_pi:f32 = PI * 2.0;
		let position:&mut [f32; 3] = self.get_position_mut();
		let rotation:[f32; 3] = [rotation[0] % double_pi, rotation[1] % double_pi, rotation[2] % double_pi];

		// If an anchor is set, simply subtract that position from the current one to shift the vertex to local space.
		if let Some(anchor_position) = anchor {
			position.displace(&anchor_position.negative());
		}

		// Calculate the sin and cos of each rotation axis.
		// As we will be doing 2 separate rotations of half the angle, calculate the cosses and sins of half of the rotation.
		let sins:Vec<f32> = rotation.iter().map(|r| (r * 0.5).sin()).collect::<Vec<f32>>();
		let coss:Vec<f32> = rotation.iter().map(|r| (r * 0.5).cos()).collect::<Vec<f32>>();

		// Calculating the angle to rotate the vertex around the virtual axis created later.
		// By multiplying the effect each angle has on their local relative X axis, it combines the effects to a single effect along the virtual axis.
		// By adding that to the effect each angle has on their local relative Y axis, we get the full size of the singular angle.
		// This is the angle we can use to rotate the 3D vertex around the axis calculated later.
		// By taking the negative of the result, it changes from rotating counter-clockwise to clockwise.
		let full_axis_angle:f32 = -(coss[0] * coss[1] * coss[2]   +   sins[0] * sins[1] * sins[2]);

		// Calculate the virtual axis to rotate around.
		// 	As shown in the table below, each axis of rotation has effect on 2 axes.
		// 	The two axes the rotation will have effect on are the 2 axes that are not the axis of rotation.
		// 	The order of the axes of effect can depend on your perspective.
		// 	To keep things stable, i will keep a pre-defined order of axis.
		// 	The easiest way to go about it in my opinion would be to use axes[(axis_index + 1) % 3] and axes[(axis_index + 2) % 3].
		//
		// 	| Global axis | local X | local Y |
		// 	| ----------- | ------- | ------- |
		// 	| X           | Y       | Z       |
		// 	| Y           | Z       | X       |
		// 	| Z           | Y       | X       |
		let virtual_axis:[f32; 3] = (0..3).map(|axis| {

			// Just like the angle, we combine the effects on each axis into one effect.
			// However, by flipping the sind and cos of the primary axis, it accounts for the orientation change.
			let local_x_effect:f32 = sins[axis] * coss[(axis + 1) % 3] * coss[(axis + 2) % 3];
			let local_y_effect:f32 = coss[axis] * sins[(axis + 1) % 3] * sins[(axis + 2) % 3];

			// The rotation system is based on a series of rotations around a 2D graph starting on the X axis towards the Y axis.
			// For this reason, the local Y effects should be subtracted here to tend to the counter-clockwise rotation on that axis.
			// The Y axis should be flipped because of the reason mentioned at the bottom of 'Calculating a rotation of a coordinate in a 2D graph'.
			local_x_effect + if axis == 1 { local_y_effect } else { -local_y_effect }
		}).collect::<Vec<f32>>().try_into().unwrap();

		// This will rotate the vertex around the calculated axis by half the required angle.
		// This rotation will produce the side-effect of the vertex being scaled up.
		// By a scaled vertex, i mean that the distance from 'position' to [0.0; 3] will increase.
		// This is caused by the full axis angle being present in some of the multiplications.
		// While it seems like the result is incorrect, it seems unnecessary to find a fix for this as it will be scaled back down in the following expression.
		// This is because aside from the scaling, the system also only does half of the rotation.
		// This means that it is possible and very useful to simply do the multiplication again with a negated version of the effector.
		// By using a version of the effector with the same angle, but negative effects, the result will both shrink down and complete the second half of the rotation.
		//
		// As the vertex that will be rotated around the virtual axis does not contain an angle itself, the angle is considered 0.0.
		// As multiplications by 0.0 are not very useful, the entire multiplications could be replaced by 0.0.
		// I chose not too for the time being as it provides clarity in the scaling process.
		// It should be removed once the method has proven to be reliable, along with some optimization.
		//
		// This calculation acts as a 4x4 multiplication matrix of which all results in a row are summed into one result.
		// The multiplications cross-combine two 4-dimensional objects, which will result in a new, combined 4-dimensional.
		// The basic idea is to have a row for each axis of the 4-dimensional model.
		// For each row, its new value will be the original vector modified by the sum of all effects over the virtual axis that take place on the 4-dimensional's model axis that belongs to the row.
		//
		// You can see some of the results of multiplications should be negated.
		// This means that the 4-dimensional models in such a multiplication matrix cannot be flipped.
		// A * B  is not necessarily equal to B * a in the case of this matrix.
		// The vertex that we are ultimately rotating around the virtual axis should not move backwards along the angle, so the angle does not need any negative multiplications.
		// Aside from that, each row should have a negative multiplication when the axis of the vertex that matches the row index is used in the multiplication.
		// Each of the last 3 axes in the 4-dimensional result should have one negative outcome because those the three values should square to -1.
		// If the squared value of the 3 axes would be 1 instead of -1, the scaling of the multiplications would be very off, especially the second considering it makes use of the first multiplications' result.
		// 
		// For each of the three rows, the negative number should be the local X axis (a rotation around the X axis has a horizontal axis Y).
		// 
		// While there are multiple possible combinations for the multiplication matrix, this one seems to be the clearest one to use as all rows have clear patterns.
		//   |  R |  X |  Y |  Z |
		//   | -- | -- | -- | -- |
		// R |  1 | -X | -Y | -Z |
		// X |  X |  1 |  Z | -Y |
		// Y |  Y | -Z |  1 |  X |
		// Z |  Z |  Y | -X |  1 |
		let vertex_angl:f32 = 0.0;
		let up_scaled_vertex:[f32; 4] = [
			full_axis_angle * vertex_angl   +  -(virtual_axis[0] * position[0])   +  -(virtual_axis[1] * position[1])   +  -(virtual_axis[2] * position[2]),
			full_axis_angle * position[0]   +   (virtual_axis[0] * vertex_angl)   +   (virtual_axis[1] * position[2])   +  -(virtual_axis[2] * position[1]),
			full_axis_angle * position[1]   +  -(virtual_axis[0] * position[2])   +   (virtual_axis[1] * vertex_angl)   +   (virtual_axis[2] * position[0]),
			full_axis_angle * position[2]   +   (virtual_axis[0] * position[1])   +  -(virtual_axis[1] * position[0])   +   (virtual_axis[2] * vertex_angl)
		];

		// Do the same thing again, but with reversed effects.
		// While the scale is negative, the angle is not, so the second half of the rotation will take place here, ensuring the complete rotation around the calculated axis.
		// Additionally, this should scale down the vertex back from its bigger variant to it's original distance from [0.0; 3] due to the negated effects.
		let down_scaled_vertex:[f32; 4] = [
			up_scaled_vertex[0] *  full_axis_angle   +   -(up_scaled_vertex[1] * -virtual_axis[0])   +   -(up_scaled_vertex[2] * -virtual_axis[1])   +   -(up_scaled_vertex[3] * -virtual_axis[2]),
        		up_scaled_vertex[0] * -virtual_axis[0]   +    (up_scaled_vertex[1] *  full_axis_angle)   +    (up_scaled_vertex[2] * -virtual_axis[2])   +   -(up_scaled_vertex[3] * -virtual_axis[1]),
        		up_scaled_vertex[0] * -virtual_axis[1]   +   -(up_scaled_vertex[1] * -virtual_axis[2])   +    (up_scaled_vertex[2] *  full_axis_angle)   +    (up_scaled_vertex[3] * -virtual_axis[0]),
        		up_scaled_vertex[0] * -virtual_axis[2]   +    (up_scaled_vertex[1] * -virtual_axis[1])   +   -(up_scaled_vertex[2] * -virtual_axis[0])   +    (up_scaled_vertex[3] *  full_axis_angle)
		];
		
		// Store the newly found location as the position of this vertex.
		*position = [down_scaled_vertex[1], down_scaled_vertex[2], down_scaled_vertex[3]];

		// If an anchor was set, add that position back into it to get the global position back from local space.
		if let Some(anchor_position) = anchor {
			position.displace(anchor_position);
		}
	}

	/// Return a rotated version of the vertex.
	fn rotated(&self, rotation:&[f32; 3], anchor:&Option<[f32; 3]>) -> Vertex {
		let mut new_pos:Vertex = self.clone_position();
		new_pos.rotate(rotation, anchor);
		new_pos
	}

	/// Create a set of data used to rotate more efficiently. More detailed comments on the maths can be found in the 'rotate' method.
	fn create_rotation_dataset(&self, rotation:&[f32; 3]) -> (f32, [f32; 3]) {
		use std::f32::consts::PI;

		let double_pi:f32 = PI * 2.0;
		let rotation:[f32; 3] = [rotation[0] % double_pi, rotation[1] % double_pi, rotation[2] % double_pi];


		// Calculate the sin and cos of each rotation axis.
		let sins:Vec<f32> = rotation.iter().map(|r| (r * 0.5).sin()).collect::<Vec<f32>>();
		let coss:Vec<f32> = rotation.iter().map(|r| (r * 0.5).cos()).collect::<Vec<f32>>();

		// Calculating the angle to rotate the vertex around the virtual axis created later.
		let full_axis_angle:f32 = -(coss[0] * coss[1] * coss[2]   +   sins[0] * sins[1] * sins[2]);

		// Calculate the virtual axis to rotate around.
		let virtual_axis:[f32; 3] = (0..3).map(|axis| {
			let local_x_effect:f32 = sins[axis] * coss[(axis + 1) % 3] * coss[(axis + 2) % 3];
			let local_y_effect:f32 = coss[axis] * sins[(axis + 1) % 3] * sins[(axis + 2) % 3];
			local_x_effect + if axis == 1 { local_y_effect } else { -local_y_effect }
		}).collect::<Vec<f32>>().try_into().unwrap();

		(full_axis_angle, virtual_axis)
	}

	/// Rotate with an earlier created dataset. More detailed comments on the maths can be found in the 'rotate' method.
	fn rotate_using_dataset(&mut self, dataset:&(f32, [f32; 3]), anchor:&Option<[f32; 3]>) {
		let (full_axis_angle, virtual_axis) = dataset;

		let position:&mut [f32; 3] = self.get_position_mut();
		
		// If an anchor is set, simply subtract that position from the current one to shift the vertex to local space.
		if let Some(anchor_position) = anchor {
			position.displace(&anchor_position.negative());
		}
		
		// This will rotate the vertex around the calculated axis by half the required angle.
		let vertex_angl:f32 = 0.0;
		let up_scaled_vertex:[f32; 4] = [
			full_axis_angle * vertex_angl   +  -(virtual_axis[0] * position[0])   +  -(virtual_axis[1] * position[1])   +  -(virtual_axis[2] * position[2]),
			full_axis_angle * position[0]   +   (virtual_axis[0] * vertex_angl)   +   (virtual_axis[1] * position[2])   +  -(virtual_axis[2] * position[1]),
			full_axis_angle * position[1]   +  -(virtual_axis[0] * position[2])   +   (virtual_axis[1] * vertex_angl)   +   (virtual_axis[2] * position[0]),
			full_axis_angle * position[2]   +   (virtual_axis[0] * position[1])   +  -(virtual_axis[1] * position[0])   +   (virtual_axis[2] * vertex_angl)
		];

		// Do the same thing again, but with reversed effects.
		let down_scaled_vertex:[f32; 4] = [
			up_scaled_vertex[0] *  full_axis_angle   +   -(up_scaled_vertex[1] * -virtual_axis[0])   +   -(up_scaled_vertex[2] * -virtual_axis[1])   +   -(up_scaled_vertex[3] * -virtual_axis[2]),
        		up_scaled_vertex[0] * -virtual_axis[0]   +    (up_scaled_vertex[1] *  full_axis_angle)   +    (up_scaled_vertex[2] * -virtual_axis[2])   +   -(up_scaled_vertex[3] * -virtual_axis[1]),
        		up_scaled_vertex[0] * -virtual_axis[1]   +   -(up_scaled_vertex[1] * -virtual_axis[2])   +    (up_scaled_vertex[2] *  full_axis_angle)   +    (up_scaled_vertex[3] * -virtual_axis[0]),
        		up_scaled_vertex[0] * -virtual_axis[2]   +    (up_scaled_vertex[1] * -virtual_axis[1])   +   -(up_scaled_vertex[2] * -virtual_axis[0])   +    (up_scaled_vertex[3] *  full_axis_angle)
		];
		
		// Store the newly found location as the position of this vertex.
		*position = [down_scaled_vertex[1], down_scaled_vertex[2], down_scaled_vertex[3]];

		// If an anchor was set, add that position back into it to get the global position back from local space.
		if let Some(anchor_position) = anchor {
			position.displace(anchor_position);
		}
	}

	/// Euler rotate this vertex in-place.
	fn euler_rotate(&mut self, rotation:&[f32; 3], anchor:&Option<[f32; 3]>) {
		use std::f32::consts::PI;

		let anchor_position:[f32; 3] = match anchor { Some(a) => *a, None => [0.0; 3] };

		// Take and shorten position variables.
		let position:&mut [f32; 3] = self.get_position_mut();

		// Translate the vertex to the origin by subtracting the anchor position.
		position.displace(&anchor_position.negative());

		// Rotate over each axis.
		let importance_axes:[[usize; 2]; 3] = [[1, 2], [0, 2], [0, 1]];
		for axis in 0..3 {
			if rotation[axis] == 0.0 { continue; }

			// As this is a rotation over 1 axis at a time, this is a 2 dimensional calculation. For each axis, an other set of axis are used. This maps the correct 3D axes to the correct 2D axes to do the calculation with.
			let horizontal_axis:usize = importance_axes[axis][0];
			let vertical_axis:usize = importance_axes[axis][1];
			let x:f32 = position[horizontal_axis];
			let y:f32 = position[vertical_axis];

			// Calculate distance and angle from origin.
			let distance:f32 = (y.abs().powi(2) + x.abs().powi(2)).sqrt();
			if distance == 0.0 { continue; }
			let mut current_angle:f32 = (x.abs() / distance).asin();
			if y < 0.0 { current_angle = PI - current_angle; }
			if x < 0.0 { current_angle = PI * 2.0 - current_angle; }

			// Calculate new angle and position.
			let new_angle:f32 = current_angle + rotation[axis];
			position[horizontal_axis] = new_angle.sin() * distance;
			position[vertical_axis] = new_angle.cos() * distance;
		}
	
		// Translate the vertex back to its original position by adding the anchor position.
		position.displace(&anchor_position);

	}

	/// Return euler a rotated version of the vertex.
	fn euler_rotated(&self, rotation:&[f32; 3], anchor:&Option<[f32; 3]>) -> Vertex {
		let mut new_pos:Vertex = self.clone_position();
		new_pos.euler_rotate(rotation, anchor);
		new_pos
	}



	/* SCALE METHODS */

	/// Displace the vertex in-place.
	fn scale(&mut self, scale:&[f32; 3]) {
		let pos:&mut Vertex = self.get_position_mut();
		for axis in 0..3 {
			pos[axis] *= scale[axis];
		}
	}

	/// Return a displaced version of the vertex.
	fn scaled(&self, scale:&[f32; 3]) -> Vertex {
		let current_position:&Vertex = self.get_position();
		[current_position[0] * scale[0], current_position[1] * scale[1], current_position[2] * scale[2]]
	}
}