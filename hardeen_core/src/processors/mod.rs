use crate::graph::*;
use crate::geometry::*;

use crate::handled_vec::Handle;
use crate::hardeen_error::HardeenError;

use std::rc::Rc;
use std::vec::Vec;
use rand::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::iter;
use itertools::Itertools;


impl Graph<GeometryWorld> {

    pub fn add_processor_node_by_type(&mut self, processor_type: &str) -> NodeHandle<GeometryWorld> {
        let handle = match processor_type {
            "Empty" => {
                let node = crate::processors::Empty::new();
                self.add_processor_node(Box::from(node))
            }
            "CreateRectangle" => {
                let node = crate::processors::CreateRectangle::new();
                self.add_processor_node(Box::from(node))
            },
            "ScatterPoints" => {
                let node = crate::processors::ScatterPoints::new();
                self.add_processor_node(Box::from(node))
            },
            "Scale" => {
                let node = crate::processors::Scale::new();
                self.add_processor_node(Box::from(node))
            },
            "RandomTangents" => {
                let node = crate::processors::RandomTangents::new();
                self.add_processor_node(Box::from(node))
            },
            "SmoothTangents" => {
                let node = crate::processors::SmoothTangents::new();
                self.add_processor_node(Box::from(node))
            },
            "AddPoints" => {
                let node = crate::processors::AddPoints::new();
                self.add_processor_node(Box::from(node))
            },
            "Merge" => {
                let node = crate::processors::Merge::new();
                self.add_processor_node(Box::from(node))
            },
            "CopyPointsAndOffset" => {
                let node = crate::processors::CopyPointsAndOffset::new();
                self.add_processor_node(Box::from(node))
            },
            "SortPointsX" => {
                let node = crate::processors::SortPointsX::new();
                self.add_processor_node(Box::from(node))
            },
            "CreateShapeFromGroup" => {
                let node = crate::processors::CreateShapeFromGroup::new();
                self.add_processor_node(Box::from(node))
            },
            "CreateShapeFromAllGroups" => {
                let node = crate::processors::CreateShapeFromAllGroups::new();
                self.add_processor_node(Box::from(node))
            },
            "Translate" => {
                let node = crate::processors::Translate::new();
                self.add_processor_node(Box::from(node))
            },
            "RandomTranslate" => {
                let node = crate::processors::RandomTranslate::new();
                self.add_processor_node(Box::from(node))
            },
            "CopyPointsAndRandomOffset" => {
                let node = crate::processors::CopyPointsAndRandomOffset::new();
                self.add_processor_node(Box::from(node))
            },
            "InstanceOnPoints" => {
                let node = crate::processors::InstanceOnPoints::new();
                self.add_subgraph_processor_node(Box::from(node))
            }
            &_ => panic!("Invalid Type-Name provided!")
        };

        handle
    }

}

macro_rules! parse_input_behaviour {
    (SlottedInput, $number_of_inputs:expr) => {
        InputBehaviour::Slotted(SlottedInput::new($number_of_inputs))
    };
    (MultipleInput, $accept_zero:expr) => {
        InputBehaviour::Multiple(MultipleInput::new($accept_zero))
    };
}

macro_rules! parse_input_behaviour_type {
    (SlottedInput, $number_of_inputs:expr) => {
        ProcessorInputType::Slotted{number_of_slots: $number_of_inputs}
    };
    (MultipleInput, $accept_zero:expr) => {
        ProcessorInputType::Multiple{zero_allowed: $accept_zero}
    };
}

macro_rules! create_processor {
    ( $type:ident, ($input_behaviour:tt, $input_behaviour_parameter:expr), $inputs:expr, [ $( $param:ident => ($param_type:tt, $param_default:expr) ),* ] ) => {
        impl $type {
            pub fn get_processor_type_info () -> ProcessorTypeInfo {               

                ProcessorTypeInfo::new(
                    stringify!($type),
                    parse_input_behaviour_type!($input_behaviour, $input_behaviour_parameter),
                    vec![
                    $(
                        ProcessorParameter { param_name: stringify!($param), param_type: stringify!($param_type) },
                    )*
                    ]
                )
            }
        }

        impl ParameteredProcessor<GeometryWorld> for $type {

            fn number_inputs(&self) -> usize {
                $inputs
            }

            fn build_input_behaviour(&self) -> InputBehaviour<NodeHandle<GeometryWorld>> {
                parse_input_behaviour!($input_behaviour, $input_behaviour_parameter)
            }

            #[allow(unused_variables)]
            fn set_parameter(&mut self, parameter_name: &str, value: &str) -> Result<(), HardeenError> {
                
                match parameter_name {
                    $(
                        stringify!($param) => {
                            self.$param = value.parse::<$param_type>().unwrap();
                            Ok(())
                        },
                    )*
                    _ => Err(HardeenError::NodeParameterDoesNotExist)
                }
            }

             fn get_parameter (&self, param: &str) -> Result<String, HardeenError> {
                match param {
                    $(
                        stringify!($param) => {
                            Ok(self.$param.to_string())
                        },
                    )*
                    _ => Err(HardeenError::NodeParameterDoesNotExist)
                } 
            }


            fn get_parameters (&self) -> &[ProcessorParameter] {
                static P : &'static [ProcessorParameter] = &[
                    $(
                        ProcessorParameter { param_name: stringify!($param), param_type: stringify!($param_type) },
                    )*
                ];
                P
            }

            fn is_parameter(&self, param: &str) -> bool {
                match param {
                    $(
                        stringify!($param) => {
                            true
                        },
                    )*
                    _ => false
                } 
            }

            fn get_processor_name (&self) -> &'static str {
                stringify!($type)
            }
        }
    };
}

pub struct CreateRectangle {
    position : Position,
    width: f32,
    height: f32
}

impl CreateRectangle {
    pub fn new() -> Self {
        CreateRectangle {
            position: Position(0.0,0.0),
            width: 10.0,
            height: 5.0
        }
    }

}

impl Processor<GeometryWorld> for CreateRectangle {

    fn run(&self, _input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = GeometryWorld::new();
        let rect = world.create_shape(true);
        let mut points = Vec::new();

        points.push(world.create_point(Point::new_linear(Position(  self.position.0 - self.width / 2.0, 
                                                                    self.position.1 - self.height / 2.0))));
        points.push(world.create_point(Point::new_linear(Position(  self.position.0 + self.width / 2.0, 
                                                                    self.position.1 - self.height / 2.0))));
        points.push(world.create_point(Point::new_linear(Position(  self.position.0 + self.width / 2.0,
                                                                  self.position.1 + self.height / 2.0))));
        points.push(world.create_point(Point::new_linear(Position(  self.position.0 - self.width / 2.0,
                                                                    self.position.1 + self.height / 2.0))));

        world.add_points_to_shape(points, &rect);

        Rc::from(world)
    }
}

create_processor!(CreateRectangle, (MultipleInput,true), 0, [
        width => (f32, 10.0),
        height => (f32, 10.0),
        position => (Position, Position(10.0, 10.0))
]);


pub struct RandomTangents {
    strength: f32
}

impl RandomTangents {

    pub fn new() -> Self {
        RandomTangents {
            strength: 2.0
        }
    }

}

impl Processor<GeometryWorld> for RandomTangents {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();
        let mut rng = rand::thread_rng();

        world.mutate_all_points(| p: &mut Point | {
            let offset = Position((rng.gen::<f32>()-0.5)*self.strength, (rng.gen::<f32>()-0.5)*self.strength);
            p.in_tangent = offset + p.position;
            p.out_tangent = Position(offset.0 * -1.0, offset.1 * -1.0) + p.position;
        } );

        Rc::from(world)
    }
}

create_processor!(RandomTangents, (SlottedInput,1), 1, [
    strength => (f32, 2.0)
]);

pub struct SmoothTangents {
    strength: f32
}

impl SmoothTangents {

    pub fn new() -> Self {
        SmoothTangents {
            strength: 2.0
        }
    }

}

impl Processor<GeometryWorld> for SmoothTangents {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        let mut iter = (*input[0]).get_shape_handle_iterator();
        while let Some(shape_handle) = iter.next() {

            let shape = (*input[0]).get_shape(&shape_handle).unwrap();
            let vertices = shape.get_vertices();

            if vertices.len() < 3 {
                continue;
            }

            let generate_tangents = | handles: (&PointHandle,&PointHandle,&PointHandle)| {
                let p1 = (*input)[0].get_point(handles.0).unwrap();
                let p2 = (*input)[0].get_point(handles.1).unwrap();
                let p3 = (*input)[0].get_point(handles.2).unwrap();

                let mut updated_point = p2.clone();

                updated_point.out_tangent = p2.position + (p3.position - p1.position) * Position(self.strength, self.strength);
                updated_point.in_tangent = p2.position + (p1.position - p3.position) * Position(self.strength, self.strength);

                world.set_point(handles.1, updated_point).expect("Point could not be set!");
            };

            if shape.is_closed() {
                iter::once(vertices.last().unwrap())
                    .chain(vertices.iter())
                    .chain(iter::once(vertices.first().unwrap()))
                    .tuple_windows::<(_, _, _)>().for_each(generate_tangents);
            }
            else {
                vertices.iter().tuple_windows::<(_, _, _)>().for_each(generate_tangents);
            }
        }

        Rc::from(world)
    }
}

create_processor!(SmoothTangents,(SlottedInput,1), 1, [
    strength => (f32, 2.0)
]);

pub struct Scale {
    factor_x: f32,
    factor_y: f32,
}

impl Scale {
    pub fn new() -> Self {
        Scale {
            factor_x: 1.2,
            factor_y: 1.2
        }
    }

}

impl Processor<GeometryWorld> for Scale {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        world.mutate_all_points(| p: &mut Point | {
            p.position.0 = p.position.0 * self.factor_x;
            p.position.1 = p.position.1 * self.factor_y;
        } );

        Rc::from(world)
    }
}

create_processor!(Scale, (SlottedInput,1), 1, [
    factor_x => (f32, 1.2),
    factor_y => (f32, 1.2)
]);


pub struct ScatterPoints {
    pub num_points: u32,
    min_position: Position,
    max_position: Position
}

impl ScatterPoints {
    pub fn new() -> Self {
        ScatterPoints {
            num_points: 10,
            min_position: Position(-200.0,-200.0),
            max_position: Position(200.0,200.0)
        }
    }
}

impl Processor<GeometryWorld> for ScatterPoints {

    fn run(&self, _input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = GeometryWorld::new();
        let mut rng = rand::thread_rng();

        let width : f32 = self.max_position.0 - self.min_position.0;
        let height : f32 = self.max_position.1 - self.min_position.1;

        for _ in 0..self.num_points {
            let x : f32 = rng.gen::<f32>() * width + self.min_position.0 ;
            let y : f32 = rng.gen::<f32>() * height + self.min_position.1;
            let p = Point::new_linear(Position(x,y));
            world.create_point(p);
        }

        Rc::from(world)
    }
}

create_processor!(ScatterPoints, (MultipleInput,true), 0, [
    num_points => (u32, 10),
    min_position => (Position, Position(-200.0,-200.0)),
    max_position => (Position, Position(200.0,200.0))
]);


pub struct AddPoints {
    positions: PositionList
}

impl AddPoints {
    pub fn new() -> Self {
        AddPoints {
            positions: PositionList(Vec::new())
        }
    }

}

impl Processor<GeometryWorld> for AddPoints {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {

        let mut world = match input.len() {
            0 => GeometryWorld::new(),
            _ => (*input[0]).clone()
        };

        for position in self.positions.0.iter() {
            world.create_point(Point::new_linear(*position));
        }

        Rc::from(world)
    }
}

create_processor!(AddPoints, (SlottedInput,1), 1, [
    positions => ( PositionList, PositionList(Vec::new()) )
]);

pub struct Empty {

}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }
}

impl Processor<GeometryWorld> for Empty {

    fn run(&self, _input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        Rc::from(GeometryWorld::new())
    }
}

create_processor!(Empty, (MultipleInput,true), 0, []);

pub struct Merge {}

impl Merge {
    pub fn new() -> Self {
        Merge {}
    }
}

impl Processor<GeometryWorld> for Merge {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();
        let mut handle_map : HashMap<PointHandle, PointHandle> = HashMap::new();

        let mut point_iter = (*input[1]).get_point_handle_iterator();
        while let Some(point_handle) = point_iter.next() {
            let merged_point = (*input[1]).get_point(&point_handle).unwrap().clone();
            let merged_handle = world.create_point(merged_point);

            handle_map.insert(point_handle, merged_handle);
        }

        let mut shape_iter=(*input[1]).get_shape_handle_iterator();
        while let Some(shape_handle) = shape_iter.next() {
            let shape = (*input[1]).get_shape(&shape_handle).unwrap();
            let merged_handle = world.create_shape(shape.is_closed());

            for old_point_handle in shape.get_vertices().iter() {
                let merged_point_handle = &handle_map[old_point_handle];
                world.add_point_to_shape(&merged_point_handle, &merged_handle);
            }
        }

        let mut group_iter=(*input[1]).get_group_handle_iterator();
        while let Some(group_handle) = group_iter.next() {
            let group = (*input[1]).get_group(&group_handle).unwrap();

            let merged_handle = match world.get_group_by_name(&group.name) {
                Some(group_handle) => {
                    group_handle.clone()
                }
                None => {
                    world.create_group(&group.name)
                }
            };

            let point_handles = group.points.iter().map(|ph| {
                handle_map.get(&ph).expect(&format!("This point does not exist! Index: {} Generation: {}", ph.get_index(), ph.get_generation())).clone()
            }).collect();

            world.add_points_to_group(point_handles, &merged_handle);

            /*for old_point_handle in group.points.iter() {
                let merged_point_handle = handle_map.get(old_point_handle).expect(&format!("This point does not exist! Index: {} Generation: {}", old_point_handle.index, old_point_handle.generation));
                world.add_point_to_group(merged_point_handle, &merged_handle);
            }*/
        }

        Rc::from(world)
    }
}

create_processor!(Merge, (MultipleInput, true), 2, []);

pub struct CopyPointsAndOffset {
    offset_position: Position
}

impl CopyPointsAndOffset {

    pub fn new() -> Self {
        CopyPointsAndOffset {
            offset_position: Position(0.0,0.0)
        }
    }

}

impl Processor<GeometryWorld> for CopyPointsAndOffset {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        let mut iter = (*input[0]).get_point_iterator();
        while let Some(point) = iter.next() {
            let mut copied_point = point.clone();
            copied_point.position = point.position + self.offset_position;
            world.create_point(copied_point);
        }

        Rc::from(world)
    }

}

create_processor!(CopyPointsAndOffset, (SlottedInput,1), 1, [
    offset_position => (Position, Position(0.0,0.0))
]);

pub struct CopyPointsAndRandomOffset {
    min_offset: Position,
    max_offset: Position,
    group_name: String,
    group: bool,
    iterations: u32
}

impl CopyPointsAndRandomOffset {

    pub fn new() -> Self {
        CopyPointsAndRandomOffset {
            min_offset: Position(0.0,0.0),
            max_offset: Position(0.0,0.0),
            group_name: String::from("all"),
            group: true,
            iterations: 1
        }
    }

}

impl Processor<GeometryWorld> for CopyPointsAndRandomOffset {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        let group_handle = &(*input[0]).get_group_by_name(&self.group_name).unwrap();
        let group = (*input[0]).get_group(&group_handle).unwrap();

        /*
        let mut iter = (&group.points).into_iter().map(|ph| {
            (ph, (*input[0]).get_point(&ph).unwrap().clone())
        });*/

        let mut rng = rand::thread_rng();
        let dx_range : f32 = self.max_offset.0 - self.min_offset.0;
        let dy_range : f32 = self.max_offset.1 - self.min_offset.1;

        for (c, point_handle) in group.points.iter().enumerate() {

            let mut last_point = (*input[0]).get_point(&point_handle).unwrap().clone();

            let group = match self.group {
                true => {
                    let group_handle = world.create_group(&format!("cg{}", c));
                    world.add_point_to_group(&point_handle, &group_handle);
                    Some(group_handle)
                },
                false => None
            };

            for _ in 0..self.iterations {
                let mut copied_point = last_point.clone();

                let dx : f32 = rng.gen::<f32>() * dx_range + self.min_offset.0 ;
                let dy : f32 = rng.gen::<f32>() * dy_range + self.min_offset.1;
                copied_point.position = last_point.position + Position(dx,dy);
                let point_handle = world.create_point(copied_point.clone());
                last_point = copied_point;

                if let Some(group_handle) = group.as_ref() {
                    world.add_point_to_group(&point_handle, group_handle);
                }

            }
        }

        Rc::from(world)
    }

}

create_processor!(CopyPointsAndRandomOffset, (SlottedInput,1), 1, [
    min_offset => (Position, Position(0.0,0.0)),
    max_offset => (Position, Position(0.0,0.0)),
    group_name => (String, "all"),
    group => (bool, true),
    iterations => (u32, 1)
]);



pub struct SortPointsX {

}

impl SortPointsX {
    pub fn new() -> Self {
        SortPointsX {

        }
    }

}

impl Processor<GeometryWorld> for SortPointsX {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();
        let group_handle = (*input)[0].get_group_by_name("all").unwrap();

        let group = world.get_group_mut(&group_handle).unwrap();

        group.points.sort_by(|handle_p1, handle_p2| {

            let point1 = (*input[0]).get_point(handle_p1).unwrap();
            let point2 = (*input[0]).get_point(handle_p2).unwrap();

            if point1.position.0 < point2.position.0 {
                return Ordering::Less;
            }
            else if point1.position.0 == point2.position.0 {
                if point1.position.1 < point2.position.1 {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            }

            Ordering::Greater
        });


        Rc::from(world)
    }
}

create_processor!(SortPointsX, (SlottedInput,1), 1, []);


pub struct SortPointsY {

}

impl SortPointsY {
    pub fn new() -> Self {
        SortPointsY {

        }
    }

}

impl Processor<GeometryWorld> for SortPointsY {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();
        let group_handle = (*input)[0].get_group_by_name("all").unwrap();

        let group = world.get_group_mut(&group_handle).unwrap();

        group.points.sort_by(|handle_p1, handle_p2| {

            let point1 = (*input[0]).get_point(handle_p1).unwrap();
            let point2 = (*input[0]).get_point(handle_p2).unwrap();

            if point1.position.1 < point2.position.1 {
                return Ordering::Less;
            }
            else if point1.position.1 == point2.position.1 {
                if point1.position.0 < point2.position.0 {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            }

            Ordering::Greater
        });


        Rc::from(world)
    }
}

create_processor!(SortPointsY, (SlottedInput,1), 1, []);

pub struct CreateShapeFromGroup {
    group_name: String,
    closed: bool
}

impl CreateShapeFromGroup {
    pub fn new() -> Self {
        CreateShapeFromGroup {
            group_name: String::from("all"),
            closed: false
        }
    }

}

impl Processor<GeometryWorld> for CreateShapeFromGroup {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {

        let mut world = (*input[0]).clone();

        let shape_handle = world.create_shape(self.closed);
        let group_handle = (*input[0]).get_group_by_name(&self.group_name).unwrap();
        let group = (*input[0]).get_group(&group_handle).unwrap();

        world.add_points_to_shape(group.points.clone(), &shape_handle);

        Rc::from(world)
    }
}

create_processor!(CreateShapeFromGroup, (SlottedInput,1), 1, [
    group_name => (String, "all"),
    closed => (bool, false)
]);

pub struct CreateShapeFromAllGroups {
    closed: bool
}

impl CreateShapeFromAllGroups {
    pub fn new() -> Self {
        CreateShapeFromAllGroups {
            closed: false
        }
    }

}

impl Processor<GeometryWorld> for CreateShapeFromAllGroups {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        let mut iter = (*input[0]).get_group_handle_iterator();

        while let Some(group_handle) =  iter.next() {
            let shape_handle = world.create_shape(self.closed);
            let group = (*input[0]).get_group(&group_handle).unwrap();

            if group.name == "all" {
                continue;
            }

            world.add_points_to_shape(group.points.clone(), &shape_handle);
        }

        Rc::from(world)
    }
}

create_processor!(CreateShapeFromAllGroups, (SlottedInput,1), 1, [
    closed => (bool, false)
]);

pub struct Translate {
    offset: Position,
    group_name: String
}

impl Translate {
    pub fn new() -> Self {
        Translate {
            offset: Position(0.0,0.0),
            group_name: String::from("all")
        }
    }

}

impl Processor<GeometryWorld> for Translate {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();
        let group_handle = &world.get_group_by_name(&self.group_name).unwrap();

        world.mutate_all_points_in_group(group_handle, |p| {
            p.position = p.position + self.offset;
        });

        Rc::from(world)
    }
}

create_processor!(Translate, (SlottedInput,1), 1, [
    offset => (Position, Position(0.0,0.0)),
    group_name => (String, "all")
]);

pub struct RandomTranslate {
    min_offset: Position,
    max_offset: Position,
    group_name: String
}

impl RandomTranslate {
    pub fn new() -> Self {
        RandomTranslate {
            min_offset: Position(0.0,0.0),
            max_offset: Position(0.0,0.0),
            group_name: String::from("all")
        }
    }

}

impl Processor<GeometryWorld> for RandomTranslate {

    fn run(&self, input : Vec<Rc<GeometryWorld>>) -> Rc<GeometryWorld> {
        let mut world = (*input[0]).clone();

        let mut rng = rand::thread_rng();

        let group_handle = &world.get_group_by_name(&self.group_name).unwrap();

        world.mutate_all_points_in_group(group_handle, |p| {
            let dx_range : f32 = self.max_offset.0 - self.min_offset.0;
            let dy_range : f32 = self.max_offset.1 - self.min_offset.1;


            let dx : f32 = rng.gen::<f32>() * dx_range + self.min_offset.0 ;
            let dy : f32 = rng.gen::<f32>() * dy_range + self.min_offset.1;

            p.position = p.position + Position(dx,dy);
        });

        Rc::from(world)
    }
}

create_processor!(RandomTranslate, (SlottedInput,1), 1, [
    min_offset => (Position, Position(0.0,0.0)),
    max_offset => (Position, Position(0.0,0.0)),
    group_name => (String, String::from("all"))
]);


pub struct InstanceOnPoints {
    group_name: String
}

impl InstanceOnPoints {
    pub fn new() -> Self {
        InstanceOnPoints {
            group_name: String::from("all")
        }
    }
}

impl SubgraphProcessor<GeometryWorld> for InstanceOnPoints {
    fn run(&self, input : Vec<Rc<GeometryWorld>>, subgraph: &Graph<GeometryWorld>) -> Rc<GeometryWorld> {
        let mut world = GeometryWorld::new();
        
        let instance_point_world = &(*input[0]);

        if subgraph.is_output_node_set() {
            for instance_point in instance_point_world.get_point_iterator() {

                let subgraph_result = subgraph.process_graph_output(false).unwrap();

                let mut instance = (*subgraph_result).clone();

                instance.mutate_all_points( |p| {
                    p.position = p.position + instance_point.position;
                });

                world.merge(&instance);
            }
        }

        Rc::new(world)
    }
}

create_processor!(InstanceOnPoints, (SlottedInput,1), 1, [
    group_name => (String, String::from("all"))
]);