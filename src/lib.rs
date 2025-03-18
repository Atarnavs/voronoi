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
        // diagram.lines.push(vec![0.0, 0.0, 0.0, 1.0]);
        // diagram.lines.push(vec![0.0, 1.0, 1.0, 1.0]);
        // diagram.lines.push(vec![0.0, 0.0, 1.0, 0.0]);
        // diagram.lines.push(vec![1.0, 1.0, 1.0, 0.0]);
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
        let point = self.points.swap_remove(index);

        if self.diagram.len() < 1 {
            self.diagram.push(point);
            return;
        }
        let last_point_put_in = &point;
        let correction_term = 1e-12;
        let mut i = 0;
        self.diagram.sort_by(|point1: &Vec<f64>, point2| Diagram::distance_squared(point1, last_point_put_in).partial_cmp(&Diagram::distance_squared(point2, last_point_put_in)).unwrap());
        for point in self.diagram.iter() {
            println!("distance to point{} is: {}", i, Diagram::distance_squared(point, last_point_put_in));
            i += 1;
        }
        println!("point being put in is : {point:?}");
        for point in self.diagram.iter() {
                // Variable names are for the clarity of math first 
            println!("point considered: {point:?}");
            let BA = vec![point[0] - last_point_put_in[0], point[1] - last_point_put_in[1]]; // vector
            // let d = Diagram::vec_len_squared(&BA);
            let M = vec![last_point_put_in[0] + 0.5 * BA[0], last_point_put_in[1] + 0.5 * BA[1]]; // midpoint
            let MO = vec![BA[1] / (Diagram::vec_len(&BA)), -BA[0] / (Diagram::vec_len(&BA))]; // turned pi/2 rad, rescaled to a unit vector.
            // let number_of_points = self.diagram.len();
            let O1O2 = vec![M[0] + MO[0], M[1] + MO[1], M[0] - MO[0], M[1] - MO[1]]; // bisector
            // self.diagram.swap(index_of_closest_point, number_of_points - 2);
            let ndy = O1O2[1] - O1O2[3];
            let dx = O1O2[2] - O1O2[0];
            let T_i = vec![0.0, O1O2[3] + (O1O2[2] * ndy) / dx]; // y intercept;
            let W_i = vec![1.0, O1O2[3] + ndy * (O1O2[2] - 1.0) / dx]; // x = 1 intercept;
            let U_i = vec![O1O2[2] + dx * (O1O2[3]) / ndy, 0.0]; // x intercept
            let V_i = vec![O1O2[2] + dx * (O1O2[3] - 1.0) / ndy, 1.0]; // y = 1 intercept
            let S: &Vec<f64>;
            let E: &Vec<f64>;
            
            if T_i[1] > 0.0 && T_i[1] < 1.0 {S = &T_i}
            else if U_i[0] > 0.0 && U_i[0] < 1.0 {
                S = &U_i;            
            }
            else {
                S = &V_i;
            }

            if W_i[1] > 0.0 && W_i[1] < 1.0 {E = &W_i}
            else if V_i[0] > 0.0 && V_i[0] < 1.0 {
                E = &V_i;            
            }
            else {
                E = &U_i;
            }
            // up to this works
            let mut SE = vec![S[0], S[1], E[0], E[1], M[0], M[1]];
            println!("SE is: {SE:?}");
            for line in self.lines.iter() { // systematically cutting off the bisector at the intectpts with other lines
                println!("line intercected is: {line:?}");
                if SE[0] == SE[2] && SE[1] == SE[3] {
                    break;
                }
                let ndy = SE[1] - SE[3];
                let dx = SE[2] - SE[0];
                let lndy = line[1] - line[3];
                let ldx = line[2] - line[0];
                let x_I = (dx * (SE[3] - line[3]) + ndy * SE[2] - line[2] * lndy * dx / ldx) / (ndy - lndy * dx / ldx); // intercept between the new line and one of the existing ones
                let y_I = SE[3] + ndy * (SE[2] - x_I) / dx; // y component of the intercept
                println!("x_I is {x_I}");
                println!("y_I is {y_I}");
                let x_Ml = x_I;
                let y_Ml = y_I;
                let x_p = x_Ml + (last_point_put_in[1] - line[5]);
                let y_p = y_Ml - (last_point_put_in[0] - line[4]);
                let dy = y_p - y_Ml;
                let dx = x_p - x_Ml;
                // println!("line intercected is {line:?}");
                let line_eqn = |x:f64, y:f64| (y - y_Ml) * dx - (x - x_Ml) * dy < 0.0;
                let mut S_n = vec![SE[0], SE[1]];
                if line_eqn(S_n[0], S_n[1]) != line_eqn(last_point_put_in[0], last_point_put_in[1]) {
                    S_n = vec![SE[2], SE[3]];
                }
                let mut x_E_n = x_I;
                let mut y_E_n = y_I;
                if ((x_I - line[0]).abs() + (x_I - line[2]).abs() - ldx.abs()).abs() > correction_term 
                || ((x_I - SE[0]).abs() + (x_I - SE[2]).abs() - dx.abs()).abs() > correction_term {
                    x_E_n = E[0];
                    y_E_n = E[1];
                }
                println!("x_E_n is: {x_E_n}\ny_E_n is: {y_E_n}");
                // if ()
                SE = vec![S_n[0], S_n[1], x_E_n, y_E_n, M[0], M[1]];
                println!("SE is: {SE:?}");
            }
            // at this point, there should be at most two intercepts lying on the line being drawn: the endpoints
            let mut i = 0;
            let mut remove_index: isize = -1;
            for line in self.lines.iter_mut() { // adjusting the other lines
                println!("line adjusted is: {line:?}");
                // let x_op = point[0] + 2.0 * line[4];
                // let y_op = point[1] + 2.0 * line[5];
                // if ((point[0] - last_point_put_in[0]).abs() + (x_op - last_point_put_in[0]).abs() - 2.0 * line[4].abs()).abs() < correction_term 
                // && ((point[1] - last_point_put_in[1]).abs() + (y_op - last_point_put_in[1]).abs() - 2.0 * line[5].abs()).abs() < correction_term {
                //     remove_index = i;
                //     continue;
                // }
                let x_I; // x component of the intercept
                let y_I;
                if ((SE[0] - line[0]) * (line[3] - line[1]) - (line[2] - line[0]) * (SE[1] - line[1])).abs() < correction_term { // if the starting point of the new line lies on the line
                    x_I = SE[0]; // set the x component of the intercept to the x component of the starting point of the old line
                    y_I = SE[1];
                }
                else if ((SE[2] - line[0]) * (line[3] - line[1]) - (line[2] - line[0]) * (SE[3] - line[1])).abs() < correction_term { // same, but for the ending point of the old line
                    x_I = SE[2];
                    y_I = SE[3];
                }
                else {
                    continue; // if there is no intercept, skip the line
                }
                if ((line[0] - x_I).abs() + (line[2] - x_I).abs() - (line[0] - line[2]).abs()).abs() > correction_term {
                    // let D = Diagram::distance_squared(&vec![line[5], line[6]], );
                    continue; // if the intecept is not on the line segment, skip.
                }
                // if SE[0] == line[0]
                let x_Ml = x_I;
                let y_Ml = y_I;
                let x_p = x_Ml + (last_point_put_in[1] - line[5]);
                let y_p = y_Ml - (last_point_put_in[0] - line[4]);
                let dy = y_p - y_Ml;
                let dx = x_p - x_Ml;
                let line_eqn = |x:f64, y:f64| {(y - y_Ml) * dx - (x - x_Ml) * dy < 0.0};
                let mut S_nl = vec![line[0], line[1]];
                if line_eqn(last_point_put_in[0],last_point_put_in[1]) == line_eqn(S_nl[0], S_nl[1]) {
                    S_nl = vec![line[2], line[3]];
                }
                *line = vec![S_nl[0], S_nl[1], x_I, y_I, x_Ml, x_Ml];
                println!("new line is: {line:?}");
                i += 1;
            }

            // if SE[2] >= 0.0 - correction_term && SE[2] <= 1.0 + correction_term && SE[3] >= 0.0 - correction_term && SE[3] <= 1.0 + correction_term && SE[0] >= 0.0 - correction_term && SE[0] <= 1.0 + correction_term && SE[1] >= 0.0 - correction_term && SE[1] <= 1.0 + correction_term {
            //     self.lines.push(SE);
            // }
            if remove_index != -1 {
                self.lines.swap_remove(remove_index as usize);
            }
            if SE[0] != SE[2] && SE[1] != SE[3] {
                self.lines.push(SE);
            }
        }
        self.diagram.push(point);
    }
    pub fn calculate_perimeters(&self) -> f64 {
        0.0
    }
    fn distance_squared(point1: &Vec<f64>, point2: &Vec<f64>) -> f64{
        (point1[0] - point2[0]).powi(2) + (point1[1] - point2[1]).powi(2)
    }
    fn vec_len(vector: &Vec<f64>) -> f64 {
        (vector[0].powi(2) + vector[1].powi(2)).sqrt()
    }
    fn vec_len_squared(vector: &Vec<f64>) -> f64 {
        vector[0].powi(2) + vector[1].powi(2)
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