// use std::{f64::consts::SQRT_2, intrinsics::sqrtf64};

use rand::{prelude::*, seq::index::sample};
pub mod info;
#[derive(Debug)]
pub struct Diagram {
    points: Vec<Vec<f64>>,
    diagram: Vec<Vec<f64>>,
    lines: Vec<Vec<f64>>
}
impl Diagram {
    pub fn new() -> Diagram {
        let diagram = Diagram {
            points: vec![],
            diagram: vec![],
            lines: vec![]
        };
        diagram
    }
    pub fn build(number: u16) -> Diagram {
        let mut diagram = Diagram::new();
        diagram.generate_points(number);
        diagram.lines.push(vec![0.0, 0.0, 0.0, 1.0]);
        diagram.lines.push(vec![0.0, 1.0, 1.0, 1.0]);
        diagram.lines.push(vec![0.0, 0.0, 1.0, 0.0]);
        diagram.lines.push(vec![1.0, 1.0, 1.0, 0.0]);
        diagram
    }
    pub fn convert_points(&self) -> Vec<Vec<u32>> {
        let mut points: Vec<Vec<u32>> = vec![];
        for point in &self.diagram {
            points.push(vec![f64::round(point[0] * 1000.0) as u32, f64::round(point[1] * 1000.0) as u32]);
        }
        points
    }
    pub fn convert_lines(&self) -> Vec<Vec<u32>> {
        let mut lines: Vec<Vec<u32>> = vec![];
        for line in &self.lines {
            lines.push(vec![f64::round(line[0] * 1000.0) as u32, f64::round(line[1] * 1000.0) as u32, f64::round(line[2] * 1000.0) as u32, f64::round(line[3] * 1000.0) as u32]);
        }
        lines
    }
    pub fn generate_points(&mut self, number: u16) {
        let mut rng = thread_rng();
        for _ in 0..number {
            self.points.push(vec![rng.gen(), rng.gen()]);
        }
    }
    pub fn put_next_point_in(&mut self) {
        if self.points.len() == 0 {
            return;
        }
        let index = thread_rng().gen_range(0..self.points.len());
        self.diagram.push(self.points.swap_remove(index));
        if self.diagram.len() < 2 {
            return;
        }
        let last_point = &self.diagram[self.diagram.len() - 1];
        let mut closest_point = &self.diagram[self.diagram.len() - 2];
        let mut distance = Diagram::distance(closest_point, last_point);
        let mut index = 0;
        let mut index_of_closest_point = 0;
        for point in self.diagram.iter().take(self.diagram.len() - 2) {
            let d = Diagram::distance(point, last_point);
            if d < distance{
                closest_point = point;
                distance = d;
                index_of_closest_point = index;
            }
            index += 1;
        }
        // Variable names are for the clarity of math first 
        let BA = vec![closest_point[0] - last_point[0], closest_point[1] - last_point[1]]; // vector
        let M = vec![last_point[0] + 0.5 * BA[0], last_point[1] + 0.5 * BA[1]]; // midpoint
        let MO = vec![BA[1] / (Diagram::vec_len(&BA)), -BA[0] / (Diagram::vec_len(&BA))]; // turned pi/2 rad, rescaled to a unit vector.
        let number_of_points = self.diagram.len();
        let mut O1O2 = vec![M[0] + MO[0], M[1] + MO[1], M[0] - MO[0], M[1] - MO[1]]; // bisector
        self.diagram.swap(index_of_closest_point, number_of_points - 2);
        for line in &self.lines {
            if (line[3] - line[1]) * MO[0] != MO[1] * (line[2] - line[0]) { // Testing if the lines intersect. No division by 0 possibility this way
                // finding the intersection point. 
                let a = O1O2[1] - O1O2[3];
                let b = O1O2[2] - O1O2[0];
                let c = line[1] - line[3];
                let d = line[2] - line[0];
                let x_I;
                let y_I;
                if d == 0.0 { // avoiding division by 0
                    x_I = (line[3] * d + line[2] * c) / c;
                    y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
                }
                else if c == 0.0 {
                    y_I = (line[3] * d + line[2] * c) / d;
                    x_I = (O1O2[3] * b + O1O2[2] * a - y_I * b) / a;
                }
                else {
                    x_I = (b * (O1O2[3] - line[3]) + O1O2[2] * a - line[2] * c * b / d) / (a - c * b / d);
                    y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
                }
                if (O1O2[1] - y_I) * (O1O2[3] - y_I) < 0.0 && (line[1] - y_I) * (line[3] - y_I) < 0.0{ // testing that the intersection lines between the points.
                    if (M[0] - x_I) * (O1O2[2] - x_I) < 0.0 { // testing which side of the midpoint the intersection is on and cutting off accordingly
                        O1O2[3] = y_I;
                        O1O2[2] = x_I;
                    }
                    else {
                        O1O2[1] = y_I;
                        O1O2[0] = x_I;
                    }
                }
            }
        }
        self.lines.push(O1O2);
        let last_point = self.diagram.last().unwrap();
        let last_line = self.lines.last().expect("must exist at this point").clone();
        for point in self.diagram.iter().take(self.diagram.len() - 2) {
            let BA = vec![point[0] - last_point[0], point[1] - last_point[1]]; // vector
            let M = vec![last_point[0] + 0.5 * BA[0], last_point[1] + 0.5 * BA[1]]; // midpoint
            let MO = vec![BA[1] / (Diagram::vec_len(&BA)), -BA[0] / (Diagram::vec_len(&BA))]; // turned pi/2 rad, rescaled to a unit vector.
            // let MO = vec![BA[1], -BA[0]];
            let mut O1O2 = vec![M[0] + MO[0], M[1] + MO[1], M[0] - MO[0], M[1] - MO[1]]; // bisector
            // finding the intersection point with the last added line;
            let a = O1O2[1] - O1O2[3];
            let b = O1O2[2] - O1O2[0];
            let c = last_line[1] - last_line[3];
            let d = last_line[2] - last_line[0];
            let x_I;
            let y_I;
            if d == 0.0 { // avoiding division by 0
                x_I = (last_line[3] * d + last_line[2] * c) / c;
                y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
            }
            else if c == 0.0 {
                y_I = (last_line[3] * d + last_line[2] * c) / d;
                x_I = (O1O2[3] * b + O1O2[2] * a - y_I * b) / a;
            }
            else {
                x_I = (b * (O1O2[3] - last_line[3]) + O1O2[2] * a - last_line[2] * c * b / d) / (a - c * b / d);
                y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
            }
            if (x_I > last_line[0] - 0.001 && x_I < last_line[0] + 0.001 && y_I < last_line[1] + 0.001 && y_I > last_line[1] - 0.001) || (x_I > last_line[2] - 0.001 && x_I < last_line[2] + 0.001 && y_I < last_line[3] + 0.001 && y_I > last_line[3] - 0.01) { // if the line intersects the last added line at the endpoint
                for line in self.lines.iter().take(self.lines.len() - 1) {
                    if (line[3] - line[1]) * MO[0] != MO[1] * (line[2] - line[0]) { // Testing if the lines intersect. No division by 0 possibility this way
                        // finding the intersection point. 
                        let a = O1O2[1] - O1O2[3];
                        let b = O1O2[2] - O1O2[0];
                        let c = line[1] - line[3];
                        let d = line[2] - line[0];
                        let x_I;
                        let y_I;
                        if d == 0.0 { // avoiding division by 0
                            x_I = (line[3] * d + line[2] * c) / c;
                            y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
                        }
                        else if c == 0.0 {
                            y_I = (line[3] * d + line[2] * c) / d;
                            x_I = (O1O2[3] * b + O1O2[2] * a - y_I * b) / a;
                        }
                        else {
                            x_I = (b * (O1O2[3] - line[3]) + O1O2[2] * a - line[2] * c * b / d) / (a - c * b / d);
                            y_I = (O1O2[3] * b + O1O2[2] * a - x_I * a) / b;
                        }
                        if (O1O2[1] - y_I) * (O1O2[3] - y_I) < 0.0 && (line[1] - y_I) * (line[3] - y_I) < 0.0{ // testing that the intersection lines between the points.
                            if (M[0] - x_I) * (O1O2[2] - x_I) < 0.0 { // testing which side of the midpoint the intersection is on and cutting off accordingly
                                O1O2[3] = y_I;
                                O1O2[2] = x_I;
                            }
                            else {
                                O1O2[1] = y_I;
                                O1O2[0] = x_I;
                            }
                        }
                    }
                }
                self.lines.push(O1O2);
            }
        }
        // for point in self.diagram.iter().take(self.diagram.len() - 1) {
        //     let BA = vec![point[0] - last_point[0], point[1] - last_point[1]];
        //     let M = vec![last_point[0] + 0.5 * BA[0], last_point[1] + 0.5 * BA[1]];
        //     let MO = vec![BA[1], -BA[0]];
        //     let O1O2 = vec![M[0] + MO[0], M[1] + MO[1], M[0] - MO[0], M[1] - MO[1]];
        //     self.lines.push(O1O2);
        // }
    }
    pub fn calculate_perimeters(&self) -> f64 {
        0.0
    }
    fn distance(point1: &Vec<f64>, point2: &Vec<f64>) -> f64{
        f64::sqrt((point1[0] - point2[0]).powi(2) + (point1[1] - point2[1]).powi(2))
    }
    fn vec_len(vector: &Vec<f64>) -> f64 {
        (vector[0].powi(2) + vector[1].powi(2)).sqrt()
    }
}
pub fn count_average_perimeters (points: &Vec<(f64, f64)>) -> f64 {
    let mut rng = thread_rng();
    let indices = sample(&mut rng, points.len(), points.len());
    let mut diagram: Vec<Vec<(f64,f64)>> = vec![vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), points[indices.index(0)]]];
    let mut perimeter = 0.0;
    let mut points_placed: Vec<(f64, f64)> = vec![points[indices.index(0)]];
    for index in indices.into_iter().skip(1) {
        let mut region_index = 0;
        let mut region_changed: &Vec<(f64,f64)> = &vec![];
        let mut closest_point_array: Vec<(f64, f64)> = vec![]; // the value doesn't matter, it'll get owerwritten, it's just here so compiler does not complain
        let mut distance = 2.0;
        let mut breakpoints: Vec<(f64, f64)> = vec![];
        for &point in points_placed.iter().skip(1) {
            if (point.1 - points[index].1).powi(2) + (point.0 - points[index].0) < distance {
                distance = (point.1 - points[index].1).powi(2) + (point.0 - points[index].0);
                closest_point_array.push(point);
            }
        }
        for (index, region) in diagram.iter().skip(1).enumerate() {
            if *region.last().expect("must exist") == closest_point_array[0] {
                region_changed = region;
                region_index = index;
                break;
            }
        }
        for closest_point in closest_point_array.into_iter().rev() {
            let u = (closest_point.0 - points[index].0, closest_point.1 - points[index].1);
            let midpoint = (points[index].0 + u.0 / 2.0, points[index].1 + u.1 / 2.0);
            // let v = (u.1, -1.0 * u.0);
            let k = -1.0 * u.1 / u.0;
            let b = midpoint.1 - k * midpoint.0;
            let sides = region_changed.len() - 1;
            for (index, point) in region_changed.iter().take(sides).enumerate() {
                let next = (index + 1) % sides;
                let k1 = (point.0 - midpoint.0) / (point.1 - midpoint.1);
                let next_point = region_changed[next];
                let k2 = (next_point.0 - midpoint.0) / (next_point.1 - midpoint.1);
                if k < k1 && k > k2 {
                    breakpoints.push((((point.1 - k * point.0) - b) / (k - ((point.0 - next_point.0) / (point.1 - next_point.1))), k * (((point.1 - k * point.0) - b) / (k - ((point.0 - next_point.0) / (point.1 - next_point.1)))) + b));
                }
            }
        }
        
        points_placed.push(points[index]);
        diagram[region_index].push(points[index]);
        perimeter += get_perimeter(diagram.last().expect("always exists"));
    }
    perimeter
}

pub fn get_perimeter(region: &Vec<(f64, f64)>) -> f64 {
    let mut perimeter = 0.0;
    for i in 1..(region.len() - 1) {
        perimeter += ((region[i - 1].1 - region[i].1).powi(2) + (region[i - 1].0 - region[i].0).powi(2)).sqrt();
    }
    perimeter
}

// macro_rules! div {

// }