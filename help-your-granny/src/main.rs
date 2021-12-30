use std::collections::HashMap;

macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() {
    println!("Help your granny!");
    let friends = ["A1", "A2", "A3", "A4", "A5"];
    let fr_towns =
        hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
    let dst =
        hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0};
    tour(&friends, fr_towns, dst);
}

fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    let mut circuit_distance: f64 = 0.0;
    let mut prev_town_distance_from_home: f64 = 0.0;
    for index in 0..frnds.len() {
        if let Some(town) = fr_twns.get(frnds[index]) {
            let distance_from_home = dist.get(town).unwrap();
            if index == 0 {
                circuit_distance += distance_from_home;
            } else if index == frnds.len() - 1 {
                let base_distance = f64::sqrt(f64::abs(
                    f64::powf(prev_town_distance_from_home, 2.0)
                        - f64::powf(*distance_from_home, 2.0),
                ));
                circuit_distance += base_distance + distance_from_home;
            } else {
                let base_distance = f64::sqrt(f64::abs(
                    f64::powf(prev_town_distance_from_home, 2.0)
                        - f64::powf(*distance_from_home, 2.0),
                ));
                circuit_distance += base_distance;
            }
            prev_town_distance_from_home = *distance_from_home;
        } else if index == 0 {
            circuit_distance += dist.get(fr_twns.get(frnds[index + 1]).unwrap()).unwrap();
        } else if index == frnds.len() - 1 {
            circuit_distance += dist.get(fr_twns.get(frnds[index - 1]).unwrap()).unwrap();
        } else {
            let next_town_dist = dist.get(fr_twns.get(frnds[index + 1]).unwrap()).unwrap();

            let base_distance = f64::sqrt(f64::abs(
                f64::powf(prev_town_distance_from_home, 2.0) - f64::powf(*next_town_dist, 2.0),
            ));
            prev_town_distance_from_home = *next_town_dist;
            circuit_distance += base_distance;
        }
    }
    println!("circuit distance => {:?}", circuit_distance);
    circuit_distance as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(
        frnds: &[&str],
        fr_twns: HashMap<&str, &str>,
        dist: HashMap<&str, f64>,
        exp: i32,
    ) -> () {
        assert_eq!(tour(&frnds, fr_twns, dist), exp)
    }

    macro_rules! hash_map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn tests_tour() {
        let friends = ["A1", "A2", "A3", "A4", "A5"];
        let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
        let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
        testing(&friends, fr_towns, dst, 889);

        let friends = ["A1", "A2", "A3", "A4", "A5"];
        let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4", "A5" => "X5" };
        let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0, "X5" => 320.0 };
        testing(&friends, fr_towns, dst, 1020);

        let friends = ["A1", "A2", "A3", "A4", "A5"];
        let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4"};
        let dst = hash_map! { "X1" => 110.0, "X2" => 210.0, "X3" => 260.0, "X4" => 360.0};
        testing(&friends, fr_towns, dst, 1051);
    }
}
