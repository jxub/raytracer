
struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}


impl Vec3 {
    fn new(&self, e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3{e0, e1, e2}
    }

    fn x(&self) -> f64 {
        self.e0
    }

    fn y(&self) -> f64 {
        self.e1
    }

    fn z(&self) -> f64 {
        self.e2
    }

    fn r(&self) -> f64 {
        self.x()
    }

    fn g(&self) -> f64 {
        self.y()
    }

    fn b(&self) -> f64 {
        self.z()
    }

    fn length(self) -> f64 {
        let values = vec![self.x(), self.y(), self.z()];
        values
            .iter()
            .map(|val| f64::powf(*val, 2.0))
            .sum::<f64>()
            .sqrt()
    }

    fn squared_length(self) -> f64 {
        let values = vec![self.x(), self.y(), self.z()];
        values
            .iter()
            .map(|val| f64::powf(*val, 2.0))
            .sum::<f64>()
    }
}

fn main() {
    chapter_one();
}

fn chapter_one() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in 0..ny {
        let j = 200 - j;
        for i in 0..nx {
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2 as f64;

            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    // cargo run --quiet > chapter1.ppm
    // open chapter1.ppm
}