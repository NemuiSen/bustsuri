use bevy::prelude::*;

pub(crate) trait Collider {
	fn get_normals(&self) -> Vec<Vec2> { vec![] } 
	fn get_positions(&self) -> Vec<Vec2>;
	fn axes_from_position(&self, other_positions: &Vec<Vec2>) -> Vec<Vec2>;
	fn range_along_axis(&self, axis_proj: Vec2) -> (f32, f32);
	fn collide(&self, other: Box<dyn Collider>) -> bool {
		let position_axes = self.axes_from_position(&other.get_positions());
		let self_normals = self.get_normals();
		let other_normals = other.get_normals();

		let axes = [position_axes, self_normals, other_normals].concat();

		for axis in axes.into_iter() {
			let (amin, amax) = self .range_along_axis(axis);
			let (bmin, bmax) = other.range_along_axis(axis);

			if !(amax >= bmin && bmax >= amin) {
				return false;
			}
		}
		true
	}
}

