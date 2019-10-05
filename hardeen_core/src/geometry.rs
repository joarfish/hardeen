use crate::handled_vec::*;
use serde::Serialize;
use std::vec::*;

use std::fmt;
use std::num::ParseFloatError;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use std::collections::HashMap;

#[derive(Clone, Serialize, Debug, Copy)]
pub struct Position(pub f32, pub f32);

impl FromStr for Position {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x = coords[0].parse::<f32>()?;
        let y = coords[1].parse::<f32>()?;

        Ok(Position(x, y))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for Position {
    type Output = Position;

    fn mul(self, other: Position) -> Position {
        Position(self.0 * other.0, self.1 * other.1)
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Point {
    pub position: Position,
    pub in_tangent: Position,
    pub out_tangent: Position,
    groups: Vec<GroupHandle>,
    shapes: Vec<ShapeHandle>,
}

impl Point {
    pub fn new(position: Position, in_tangent: Position, out_tangent: Position) -> Self {
        Point {
            position,
            in_tangent,
            out_tangent,
            groups: Vec::new(),
            shapes: Vec::new(),
        }
    }

    pub fn new_linear(position: Position) -> Self {
        Point::new(position, Position(0.0, 0.0), Position(0.0, 0.0))
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }
}

pub type PointHandle = MarkedHandle<Point>;
pub type PointDataVector = ImmutableVector<Point>;
pub type PointHandleIterator<'a> = HandleIterator<'a, PointHandle, PointDataVector>;
pub type PointIterator<'a> = DataIterator<'a, PointDataVector>;

#[derive(Clone, Serialize, Debug)]
pub struct Group {
    pub name: String,
    pub points: Vec<PointHandle>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            name: String::from(name),
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, handle: &PointHandle) {
        self.points.push(handle.clone());
    }

    pub fn add_points(&mut self, point_handles: &mut Vec<PointHandle>) {
        self.points.append(point_handles);
    }
}

pub type GroupHandle = MarkedHandle<Group>;
pub type GroupDataVector = ImmutableVector<Group>;
pub type GroupHandleIterator<'a> = HandleIterator<'a, GroupHandle, GroupDataVector>;
pub type GroupIterator<'a> = DataIterator<'a, GroupDataVector>;

#[derive(Clone, Serialize, Debug)]
pub struct Shape {
    vertices: Vec<PointHandle>,
    closed: bool,
}

impl Shape {
    pub fn new(closed: bool) -> Self {
        Shape {
            vertices: Vec::new(),
            closed,
        }
    }

    pub fn add_points(&mut self, point_handles: &mut Vec<PointHandle>) {
        self.vertices.append(point_handles);
    }

    pub fn add_point(&mut self, point_handle: &PointHandle) {
        self.vertices
            .insert(self.vertices.len(), point_handle.clone());
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn get_vertices(&self) -> &[PointHandle] {
        &self.vertices
    }
}

pub type ShapeHandle = MarkedHandle<Shape>;
pub type ShapeDataVector = ImmutableVector<Shape>;
pub type ShapeHandleIterator<'a> = HandleIterator<'a, ShapeHandle, ShapeDataVector>;
pub type ShapeIterator<'a> = DataIterator<'a, ShapeDataVector>;

#[derive(Debug)]
pub enum GeometryWorldError {
    Error(&'static str),
}

#[derive(Serialize, Clone)]
pub struct GeometryWorld {
    points: HandledVec<PointHandle, PointDataVector>,
    groups: HandledVec<GroupHandle, GroupDataVector>,
    shapes: HandledVec<ShapeHandle, ShapeDataVector>,
    all_point_group_handle: GroupHandle,
}

impl GeometryWorld {
    pub fn new() -> Self {
        let mut world = GeometryWorld {
            points: HandledVec::new(),
            groups: HandledVec::new(),
            shapes: HandledVec::new(),
            all_point_group_handle: GroupHandle::new(0, 0),
        };

        world.all_point_group_handle = world.create_group("all");

        world
    }

    pub fn create_point(&mut self, mut point: Point) -> PointHandle {
        point.groups.push(self.all_point_group_handle.clone());
        let h = self.points.add_entry(point);
        self.groups
            .get_mut(&self.all_point_group_handle)
            .unwrap()
            .add_point(&h);

        h
    }

    pub fn remove_point(&mut self, point_handle: PointHandle) -> Result<(), GeometryWorldError> {
        let point = self.points.get(&point_handle).unwrap();

        for group_handle in point.groups.iter() {
            if let Ok(group) = self.groups.get_mut(group_handle) {
                group.points.retain(|ph| ph != &point_handle);
            }
        }

        for shape_handle in point.shapes.iter() {
            if let Ok(shape) = self.shapes.get_mut(shape_handle) {
                shape.vertices.retain(|ph| ph != &point_handle);
            }
        }

        return match self.points.remove_entry(point_handle) {
            Ok(()) => Ok(()),
            Err(_error) => Err(GeometryWorldError::Error("Couldn't remove point!")),
        };
    }

    pub fn get_point(&self, handle: &PointHandle) -> Result<&Point, HandledVecError> {
        self.points.get(handle)
    }

    pub fn set_point(
        &mut self,
        handle: &PointHandle,
        point: Point,
    ) -> Result<(), GeometryWorldError> {
        return match self.points.update(handle, point) {
            Ok(()) => Ok(()),
            Err(_error) => Err(GeometryWorldError::Error("Couldn't update point!")),
        };
    }

    pub fn create_group(&mut self, name: &str) -> GroupHandle {
        self.groups.add_entry(Group::new(name))
    }

    pub fn get_point_handle_from_index(&self, index: usize) -> PointHandle {
        self.points.get_handle_for_index(index).unwrap()
    }

    pub fn add_points_to_group(
        &mut self,
        mut point_handles: Vec<PointHandle>,
        group_handle: &GroupHandle,
    ) {
        if let Ok(group) = self.groups.get_mut(group_handle) {
            while let Some(point_handle) = point_handles.pop() {
                if let Ok(point) = self.points.get_mut(&point_handle) {
                    group.points.push(point_handle);
                    point.groups.push(group_handle.clone());
                }
            }
        }
    }

    #[inline]
    pub fn add_point_to_group(&mut self, point_handle: &PointHandle, group_handle: &GroupHandle) {
        self.add_points_to_group(vec![point_handle.clone()], group_handle);
    }

    pub fn get_point_handle_iterator(&self) -> PointHandleIterator {
        self.points.get_handle_iterator()
    }
    pub fn get_point_iterator(&self) -> PointIterator {
        self.points.get_iterator()
    }

    pub fn get_shape_handle_iterator(&self) -> ShapeHandleIterator {
        self.shapes.get_handle_iterator()
    }

    pub fn get_point_count(&self) -> usize {
        self.points.get_length()
    }

    pub fn create_shape(&mut self, closed: bool) -> ShapeHandle {
        self.shapes.add_entry(Shape::new(closed))
    }

    pub fn get_shape(&self, handle: &ShapeHandle) -> Result<&Shape, HandledVecError> {
        self.shapes.get(handle)
    }

    pub fn get_group(&self, handle: &GroupHandle) -> Result<&Group, HandledVecError> {
        self.groups.get(handle)
    }

    pub fn get_group_mut(&mut self, handle: &GroupHandle) -> Result<&mut Group, HandledVecError> {
        self.groups.get_mut(handle)
    }

    pub fn add_points_to_shape(
        &mut self,
        mut point_handles: Vec<PointHandle>,
        shape_handle: &ShapeHandle,
    ) {
        if let Ok(shape) = self.shapes.get_mut(shape_handle) {
            while let Some(point_handle) = point_handles.pop() {
                if let Ok(point) = self.points.get_mut(&point_handle) {
                    shape.vertices.push(point_handle);
                    point.shapes.push(shape_handle.clone());
                }
            }
            //shape.add_points(point_handles);
        }
    }

    pub fn add_point_to_shape(&mut self, point_handle: &PointHandle, shape_handle: &ShapeHandle) {
        if let Ok(_shape) = self.shapes.get_mut(shape_handle) {
            self.add_points_to_shape(vec![point_handle.clone()], shape_handle);
        }
    }

    pub fn get_all_points(&self) -> &[PointHandle] {
        &self
            .groups
            .get(&self.all_point_group_handle)
            .unwrap()
            .points
    }

    pub fn get_group_handle_iterator(&self) -> GroupHandleIterator {
        self.groups.get_handle_iterator()
    }

    pub fn get_group_by_name(&self, name: &str) -> Option<GroupHandle> {
        for group_handle in self.groups.get_handle_iterator() {
            let group = self.groups.get(&group_handle).expect(&format!("Group Handle is invalid: index: {}, generation: {}.", group_handle.get_generation(), group_handle.get_index()));
            if group.name == name {
                return Some(group_handle.clone());
            }
        }

        None
    }

    #[inline]
    pub fn mutate_all_points<F: FnMut(&mut Point)>(&mut self, func: F) {
        self.points.mutate_each(func);
    }

    pub fn mutate_all_points_in_group<F: FnMut(&mut Point)>(
        &mut self,
        group_handle: &GroupHandle,
        mut func: F,
    ) {
        let group = self.groups.get(group_handle).unwrap();

        for point_handle in group.points.clone().iter() {
            func(self.points.get_mut(point_handle).unwrap());
        }
    }

    pub fn merge(&mut self, other: &GeometryWorld) {

        let mut handle_map : HashMap<PointHandle, PointHandle> = HashMap::new();

        let mut point_iter = other.get_point_handle_iterator();
        while let Some(point_handle) = point_iter.next() {
            let merged_point = other.get_point(&point_handle).unwrap().clone();
            let merged_handle = self.create_point(merged_point);

            handle_map.insert(point_handle, merged_handle);
        }

        let mut shape_iter=other.get_shape_handle_iterator();
        while let Some(shape_handle) = shape_iter.next() {
            let shape = other.get_shape(&shape_handle).unwrap();
            let merged_handle = self.create_shape(shape.is_closed());

            for old_point_handle in shape.get_vertices().iter() {
                let merged_point_handle = &handle_map[old_point_handle];
                self.add_point_to_shape(&merged_point_handle, &merged_handle);
            }
        }

        let mut group_iter=other.get_group_handle_iterator();
        while let Some(group_handle) = group_iter.next() {
            let group = other.get_group(&group_handle).unwrap();

            let merged_handle = match self.get_group_by_name(&group.name) {
                Some(group_handle) => {
                    group_handle.clone()
                }
                None => {
                    self.create_group(&group.name)
                }
            };

            let point_handles = group.points.iter().map(|ph| {
                handle_map.get(&ph).expect(&format!("This point does not exist! Index: {} Generation: {}", ph.get_index(), ph.get_generation())).clone()
            }).collect();

            self.add_points_to_group(point_handles, &merged_handle);
        }

    }

    pub fn get_bounding_rect(&self) -> (Position, Position) {
        if self.points.get_length() == 0 {
            return (Position(0.0, 0.0), Position(0.0, 0.0));
        }

        let mut iter = self.points.get_iterator();

        let mut nw = iter.next().unwrap().position.clone();
        let mut se = nw.clone();

        while let Some(point) = iter.next() {
            let position = point.position.clone();

            if position.0 < nw.0 {
                nw.0 = position.0;
            }

            if position.1 < nw.1 {
                nw.1 = position.1;
            }

            if position.0 > se.0 {
                se.0 = position.0;
            }

            if position.1 > se.1 {
                se.1 = position.1;
            }
        }

        (nw.clone(), se.clone())
    }
}
