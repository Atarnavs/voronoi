// use std::{f64::consts::SQRT_2, intrinsics::sqrtf64};

use std::usize;

use rand::{prelude::*, seq::index::sample};
use log::LevelFilter;

pub mod info;

#[derive(Debug)]
pub struct Diagram {
    points: Vec<Vec<f64>>,
    diagram: Vec<Vec<f64>>,
    lines: Vec<Vec<f64>>,
}
impl Diagram {
    pub fn new() -> Diagram {
        let diagram = Diagram {
            points: vec![],
            diagram: vec![],
            lines: vec![],
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
    pub fn put_next_point_in2(&mut self) {
        if self.points.len() == 0 {
            return;
        }
        let index = thread_rng().gen_range(0..self.points.len());
        let point = self.points.swap_remove(index);

        if self.diagram.len() < 1 {
            println!("point being put in is: {point:?}");
            self.diagram.push(point);
            println!("\n\n_________________________________________________________________________________\n");
            return;
        }
        let last_point_put_in = &point;
        let correction_term = 1e-11;
        let mut point_counter = 0;
        self.diagram.sort_by(|point1: &Vec<f64>, point2| Diagram::distance_squared(point1, last_point_put_in).partial_cmp(&Diagram::distance_squared(point2, last_point_put_in)).unwrap());
        for point in self.diagram.iter() {
            println!("point is: {point:?}\ndistance to point{} is: {}", point_counter, Diagram::distance_squared(point, last_point_put_in));
            point_counter += 1;
        }
        println!("point being put in is: {point:?}");
        for other_point_in_the_diagram in self.diagram.iter() {
                // Variable names are for the clarity of math first 
            println!("point considered: {other_point_in_the_diagram:?}");
            let BA = vec![other_point_in_the_diagram[0] - last_point_put_in[0], other_point_in_the_diagram[1] - last_point_put_in[1]]; // vector
            let M = vec![last_point_put_in[0] + 0.5 * BA[0], last_point_put_in[1] + 0.5 * BA[1]]; // midpoint
            let MO = vec![BA[1] / (Diagram::vec_len(&BA)), -BA[0] / (Diagram::vec_len(&BA))]; // turned pi/2 rad, rescaled to a unit vector.
            let O1O2 = vec![M[0] + MO[0], M[1] + MO[1], M[0] - MO[0], M[1] - MO[1]]; // bisector
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
            let mut SE = vec![S[0], S[1], E[0], E[1], M[0], M[1], other_point_in_the_diagram[0], other_point_in_the_diagram[1]];
            let mut i = 0;
            let mut remove_indeces: Vec<usize> = vec![];
            let mut line_to_be_adjusted_indeces_and_intercepts: Vec<(usize,[f64; 2])> = vec![]; // (index, [x intercept, y intercept])
            println!("SE is: {SE:?}");
            let side = true;
            for line in self.lines.iter() { // systematically cutting off the bisector at the intectpts with other lines
                println!("line intercected is: {line:?}\nindex is {i}");
                if SE[0] == SE[2] && SE[1] == SE[3] {
                    i = 0;
                    line_to_be_adjusted_indeces_and_intercepts.clear();
                    break;
                }
                let ndy = SE[1] - SE[3];
                let dx = SE[2] - SE[0];
                let lndy = line[1] - line[3];
                let ldx = line[2] - line[0];
                let x_i = (dx * (SE[3] - line[3]) + ndy * SE[2] - line[2] * lndy * dx / ldx) / (ndy - lndy * dx / ldx); // intercept between the new line and one of the existing ones
                let y_i = SE[3] + ndy * (SE[2] - x_i) / dx; // y component of the intercept
                println!("x_I is {x_i}");
                println!("y_I is {y_i}");
                if (SE[0] - x_i).abs() < correction_term || (SE[2] - x_i).abs() < correction_term {
                    i += 1;
                    continue;
                }
                let x_Ml = x_i;
                let y_Ml = y_i;
                let x_p = x_Ml + (last_point_put_in[1] - line[5]);
                let y_p = y_Ml - (last_point_put_in[0] - line[4]);
                let dye = y_p - y_Ml;
                let dxe = x_p - x_Ml;
                // println!("line intercected is {line:?}");
                println!("{}, {}, {}, {}", x_Ml, y_Ml, dxe, dye);
                let line_eqn = |x:f64, y:f64| (y - y_Ml) * dxe - (x - x_Ml) * dye < 0.0;
                let mut S_n = [SE[0], SE[1]];
                println!("the start is on {} side\nthe point is on the {} side", line_eqn(S_n[0], S_n[1]), line_eqn(last_point_put_in[0], last_point_put_in[1]));
                if line_eqn(S_n[0], S_n[1]) != line_eqn(last_point_put_in[0], last_point_put_in[1]) {
                    S_n = [SE[2], SE[3]];
                    // let side = false;
                }
                let mut x_E_n = x_i;
                let mut y_E_n = y_i;
                println!("{}, {}, {}, {}\n{}, {}, {}, {}", (x_i - line[0]).abs(), (x_i - line[2]).abs(), ldx.abs(), ((x_i - line[0]).abs() + (x_i - line[2]).abs() - ldx.abs()).abs()
            , (x_i - SE[0]).abs(), (x_i - SE[2]).abs(), dx.abs(), ((x_i - SE[0]).abs() + (x_i - SE[2]).abs() - dx.abs()).abs());
                let on_se = ((x_i - SE[0]).abs() + (x_i - SE[2]).abs() - dx.abs()).abs() < correction_term;
                let on_the_line = ((x_i - line[0]).abs() + (x_i - line[2]).abs() - ldx.abs()).abs() < correction_term;
                println!("ont the line: {}, se: {}", on_the_line, on_se);
                if !on_the_line && !on_se {
                    // println!("not on th");
                    if ((line[2] - line[0]) * (line[4] - point[0]) + (line[3] - line[1]) * (line[5] - point[1])).abs() < correction_term {
                        if side {
                            x_E_n = SE[2];
                            y_E_n = SE[3];
                        }
                        else {
                            x_E_n = SE[0];
                            y_E_n = SE[1];
                        }
                    }
                    else {
                        i += 1;
                        continue;
                    }
                }
                else if on_the_line && !on_se {
                    i += 1;
                    continue;
                }
                else if !on_the_line && on_se {
                    if ((line[2] - line[0]) * (line[4] - last_point_put_in[0]) + (line[3] - line[1]) * (line[5] - last_point_put_in[1])).abs() < correction_term {
                        x_E_n = S_n[0];
                        y_E_n = S_n[1];
                    }
                    else if ((line[2] - line[0]) * (line[4] - point[0]) + (line[3] - line[1]) * (line[5] - point[1])).abs() < correction_term {
                        line_to_be_adjusted_indeces_and_intercepts.push((i, [x_i, y_i]));
                        i += 1;
                        continue;
                    }
                    else {
                        i += 1;
                        continue;
                    }
                }
                line_to_be_adjusted_indeces_and_intercepts.push((i, [x_i, y_i]));
                println!("x_E_n is: {x_E_n}\ny_E_n is: {y_E_n}");
                SE[0] = S_n[0];
                SE[1] = S_n[1];
                SE[2] = x_E_n;
                SE[3] = y_E_n;
                // SE = vec![*S_n[0], *S_n[1], x_E_n, y_E_n, M[0], M[1]];

                println!("SE is: {SE:?}");
                i += 1;
            }
            if SE[0] == SE[2] && SE[1] == SE[3] {
                line_to_be_adjusted_indeces_and_intercepts.clear();
            }
            println!("\nadjusting lines");
            for index in 0..line_to_be_adjusted_indeces_and_intercepts.len() {
                let line_to_be_adjusted = &mut self.lines[line_to_be_adjusted_indeces_and_intercepts[index].0];
                println!("line adjusted is: {line_to_be_adjusted:?}");
                let x_I = line_to_be_adjusted_indeces_and_intercepts[index].1[0];
                let y_I = line_to_be_adjusted_indeces_and_intercepts[index].1[1];
                println!("x_I is: {x_I}\ny_I is: {y_I}");
                let intercept_on_the_line = ((line_to_be_adjusted[0] - x_I).abs() + (line_to_be_adjusted[2] - x_I).abs() - (line_to_be_adjusted[2] - line_to_be_adjusted[0]).abs()).abs() < correction_term;
                let intercept_on_se = ((SE[0] - x_I).abs() + (SE[2] - x_I).abs() - (SE[2] - SE[0]).abs()) < correction_term;
                if (x_I - line_to_be_adjusted[0]).abs() < correction_term || (x_I - line_to_be_adjusted[2]).abs() < correction_term{
                    continue;
                }
                let distance_squared_from_line_start_to_its_point = (line_to_be_adjusted[0] - line_to_be_adjusted[6]).powi(2) + (line_to_be_adjusted[1] - line_to_be_adjusted[7]).powi(2);
                let distance_squared_from_line_end_to_its_point = (line_to_be_adjusted[2] - line_to_be_adjusted[6]).powi(2) + (line_to_be_adjusted[3] - line_to_be_adjusted[7]).powi(2);
                let distance_squared_from_line_start_to_new_point = (line_to_be_adjusted[0] - last_point_put_in[0]).powi(2) + (line_to_be_adjusted[1] - last_point_put_in[1]).powi(2);
                let distance_squared_from_line_end_to_new_point = (line_to_be_adjusted[2] - last_point_put_in[0]).powi(2) + (line_to_be_adjusted[3] - last_point_put_in[1]).powi(2);
                if intercept_on_the_line && intercept_on_se {
                    if distance_squared_from_line_end_to_its_point - distance_squared_from_line_end_to_new_point > correction_term {
                        line_to_be_adjusted[2] = x_I;
                        line_to_be_adjusted[3] = y_I;
                    }
                    else if distance_squared_from_line_start_to_its_point - distance_squared_from_line_start_to_new_point > correction_term  {
                        line_to_be_adjusted[0] = x_I;
                        line_to_be_adjusted[1] = y_I;                        
                    }
                }
                println!("new line is {line_to_be_adjusted:?}");
            }
            remove_indeces.sort_unstable();
            remove_indeces.dedup();
            for j in 0..remove_indeces.len() {
                self.lines.remove(remove_indeces[j]);
                remove_indeces.iter_mut().enumerate().for_each(|(i, index): (usize, &mut usize)| if i > j {*index -= 1});
            }
            // self.lines = self.lines.iter().enumerate().filter(|(i, _): &(usize, &Vec<f64>) | remove_indeces.contains(i)).map(|(_, line): (usize, &Vec<f64>)| *line).collect();
            if (SE[0] - SE[2]).abs() > correction_term && (SE[1] - SE[3]).abs() > correction_term {
                self.lines.push(SE);
            }
            println!("---------------------------------------");
        }
        let mut remove_indeces: Vec<usize> = vec![];
        let mut i = 0;
        println!("looking for leftovers");
        for line in self.lines.iter() {
            let distance_squared_line_start_to_its_point = (line[6] - line[0]).powi(2) + (line[7] - line[1]).powi(2);
            let distance_squared_line_start_to_point_added = (line[0] - last_point_put_in[0]).powi(2) + (line[1] - last_point_put_in[1]).powi(2);
            if  distance_squared_line_start_to_its_point - distance_squared_line_start_to_point_added > correction_term {
                remove_indeces.push(i);
                println!("distance to the line's point is: {} and to the point being put in is: {}", distance_squared_line_start_to_its_point, distance_squared_line_start_to_point_added);
                println!("adding {line:?} to the removal indices")
            }
            i += 1;
        }
        remove_indeces.sort_unstable();
        remove_indeces.dedup();
        for j in 0..remove_indeces.len() {
            self.lines.remove(remove_indeces[j]);
            remove_indeces.iter_mut().enumerate().for_each(|(i, index): (usize, &mut usize)| if i > j {*index -= 1});
        }
        self.diagram.push(point);
        println!("\n\n_________________________________________________________________________________\n");
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

// macro_rules! div {

// }