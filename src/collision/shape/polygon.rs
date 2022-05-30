use bevy::{prelude::*, math::vec2};
use super::Collider;

pub struct Polygon {
	pub vertices: Vec<Vec2>,
}

impl Polygon {
	pub(crate) fn from_vertices(vertices: Vec<Vec2>, transform: GlobalTransform) -> Self {
		Self {
			vertices: Self::rotate_translate(vertices, transform),
		}
	}

	pub(crate) fn square(w: f32, h: f32, transform: GlobalTransform) -> Self {
		Self {
			vertices: Self::rotate_translate(vec![
				/*
				vec2(-w, -h),
				vec2(-w,  h),
				vec2( w,  h),
				vec2( w, -h),
				*/
				vec2(-w, -h),
				vec2( w, -h),
				vec2( w,  h),
				vec2(-w,  h),
			], transform),
		}
	}

	fn rotate_translate(vertices: Vec<Vec2>, transform: GlobalTransform) -> Vec<Vec2> {
		let angle = transform.rotation.to_scaled_axis().z;
		let c = angle.cos();
		let s = angle.sin();
		let position = transform.translation.truncate();
		vertices.iter().map(|v| {
			vec2(v.x*c - v.y*s, v.x*s + v.y*c) + position
		}).collect()
	}
}

impl Collider for Polygon {
	fn get_positions(&self) -> Vec<Vec2> {
		self.vertices.clone()
	}

	fn axes_from_position(&self, other_positions: &Vec<Vec2>) -> Vec<Vec2> {
		if other_positions.len() > 1 {
			return vec![];
		}

		self.vertices.iter()
			.map(move |&v| (other_positions[0]-v)
			.normalize_or_zero())
			.collect()
	}

	fn get_normals(&self) -> Vec<Vec2> {
		let sz = self.vertices.len();
		(0..sz).map(move |i| {
			let p1 = self.vertices[i];
			let p2 = self.vertices[(i+1)%sz];
			vec2(-(p2.y-p1.y), p2.x-p1.x).normalize_or_zero()
		}).collect()
	}

	fn range_along_axis(&self, axis_proj: Vec2) -> (f32, f32) {
		let (mut min, mut max) = (f32::INFINITY, -f32::INFINITY);
		for vertex in &self.vertices {
			let d = vertex.dot(axis_proj);
			min = min.min(d);
			max = max.max(d);
		}
		(min, max)
	}
}

