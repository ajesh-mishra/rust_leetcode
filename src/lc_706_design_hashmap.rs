struct MyHashMap {
    keys: Vec<Option<i32>>,
    values: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            keys: vec![],
            values: vec![],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(pos) = self
            .keys
            .iter()
            .position(|&x| x.is_some() && x.unwrap() == key)
        {
            self.values[pos] = value;
        } else {
            self.keys.push(Some(key));
            self.values.push(value);
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(pos) = self
            .keys
            .iter()
            .position(|&x| x.is_some() && x.unwrap() == key)
        {
            self.values[pos]
        } else {
            -1
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(pos) = self
            .keys
            .iter()
            .position(|&x| x.is_some() && x.unwrap() == key)
        {
            self.keys[pos] = None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut() {
        let operations = [
            "MyHashMap",
            "remove",
            "put",
            "remove",
            "remove",
            "get",
            "remove",
            "put",
            "get",
            "remove",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "remove",
            "put",
            "put",
            "get",
            "put",
            "get",
            "put",
            "put",
            "get",
            "put",
            "remove",
            "remove",
            "put",
            "put",
            "get",
            "remove",
            "put",
            "put",
            "put",
            "get",
            "put",
            "put",
            "remove",
            "put",
            "remove",
            "remove",
            "remove",
            "put",
            "remove",
            "get",
            "put",
            "put",
            "put",
            "put",
            "remove",
            "put",
            "get",
            "put",
            "put",
            "get",
            "put",
            "remove",
            "get",
            "get",
            "remove",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "get",
            "get",
            "remove",
            "put",
            "put",
            "put",
            "put",
            "get",
            "remove",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "put",
            "remove",
            "remove",
            "get",
            "remove",
            "put",
            "put",
            "remove",
            "get",
            "put",
            "put",
        ];
        let parameters = [
            vec![1],
            vec![27],
            vec![65, 65],
            vec![19],
            vec![0],
            vec![18],
            vec![3],
            vec![42, 0],
            vec![19],
            vec![42],
            vec![17, 90],
            vec![31, 76],
            vec![48, 71],
            vec![5, 50],
            vec![7, 68],
            vec![73, 74],
            vec![85, 18],
            vec![74, 95],
            vec![84, 82],
            vec![59, 29],
            vec![71, 71],
            vec![42],
            vec![51, 40],
            vec![33, 76],
            vec![17],
            vec![89, 95],
            vec![95],
            vec![30, 31],
            vec![37, 99],
            vec![51],
            vec![95, 35],
            vec![65],
            vec![81],
            vec![61, 46],
            vec![50, 33],
            vec![59],
            vec![5],
            vec![75, 89],
            vec![80, 17],
            vec![35, 94],
            vec![80],
            vec![19, 68],
            vec![13, 17],
            vec![70],
            vec![28, 35],
            vec![99],
            vec![37],
            vec![13],
            vec![90, 83],
            vec![41],
            vec![50],
            vec![29, 98],
            vec![54, 72],
            vec![6, 8],
            vec![51, 88],
            vec![13],
            vec![8, 22],
            vec![85],
            vec![31, 22],
            vec![60, 9],
            vec![96],
            vec![6, 35],
            vec![54],
            vec![15],
            vec![28],
            vec![51],
            vec![80, 69],
            vec![58, 92],
            vec![13, 12],
            vec![91, 56],
            vec![83, 52],
            vec![8, 48],
            vec![62],
            vec![54],
            vec![25],
            vec![36, 4],
            vec![67, 68],
            vec![83, 36],
            vec![47, 58],
            vec![82],
            vec![36],
            vec![30, 85],
            vec![33, 87],
            vec![42, 18],
            vec![68, 83],
            vec![50, 53],
            vec![32, 78],
            vec![48, 90],
            vec![97, 95],
            vec![13, 8],
            vec![15, 7],
            vec![5],
            vec![42],
            vec![20],
            vec![65],
            vec![57, 9],
            vec![2, 41],
            vec![6],
            vec![33],
            vec![16, 44],
            vec![95, 30],
        ];
        let expected_results: [i32; 101] = [
            10101, 10101, 10101, 10101, 10101, -1, 10101, 10101, -1, 10101, 10101, 10101, 10101,
            10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 90, 10101,
            -1, 10101, 10101, 40, 10101, 10101, 10101, 10101, 10101, 29, 10101, 10101, 10101,
            10101, 17, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 33, 10101,
            10101, 10101, 10101, 10101, 10101, 18, 10101, 10101, -1, 10101, 10101, -1, 35, 10101,
            10101, 10101, 10101, 10101, 10101, 10101, -1, -1, 10101, 10101, 10101, 10101, 10101,
            -1, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101, 10101,
            10101, -1, 10101, 10101, 10101, 10101, 87, 10101, 10101,
        ];

        let mut map = MyHashMap::new();

        for i in 0..101 {
            match operations[i] {
                "remove" => {
                    map.remove(parameters[i][0]);
                }
                "put" => {
                    map.put(parameters[i][0], parameters[i][1]);
                }
                "get" => {
                    dbg!(&map.keys);
                    dbg!(&map.values);
                    dbg!(parameters[i][0]);
                    assert_eq!(map.get(parameters[i][0]), expected_results[i]);

                    // println!("{}", expected_results[i]);
                    // println!("{}\n", map.get(parameters[i][0]));
                }
                _ => {}
            }
        }

        // let mut map = MyHashMap::new();
        // map.remove(27);
        // map.put(65, 65);
        // map.remove(18);
        // map.remove(0);
        // assert_eq!(map.get(18), -1);
        // map.remove(3);
        // map.put(42, 0);
        // assert_eq!(map.get(19), -1);
        // map.remove(42);
        //
        // map.put(31, 76);
        // map.put(48, 71);
        // map.put(5, 50);
        // map.put(7, 68);
        //
        // map.put(73, 74);
        // map.put(85, 18);
        // map.put(74, 95);
        // map.put(84, 82);
        // map.put(59, 29);
        //
        // map.put(71, 71);
        // map.remove(42);
        //
        // map.put(51, 40);
        // map.put(33, 76);
        // assert_eq!(map.get(17), 90);
        //
        // map.put(59, 29);
        // map.put(59, 29);
        // map.put(59, 29);
    }
}
