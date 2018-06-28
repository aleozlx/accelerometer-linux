use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

struct Accelerometer<'a> {
    base: &'a Path,
    scale: f32,
}

impl<'a> Accelerometer<'a> {
    fn init(&mut self) -> std::io::Result<()> {
        let fname_scale = self.base.join("in_accel_scale");
        let mut f = File::open(fname_scale).expect("IO Error");
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        self.scale = content.trim().parse().unwrap();
        Ok(())
    }

    fn open(&self, axis: &str) -> std::io::Result<File> {
        let fname_accel = self.base.join(format!("in_accel_{}_raw", axis));
        File::open(fname_accel)
    }

    fn read(&self, f_sensor: &mut File) -> std::io::Result<f32> {
        let mut content = String::new();
        f_sensor.seek(SeekFrom::Start(0))?;
        f_sensor.read_to_string(&mut content)?;
        let val = content.trim().parse::<f32>().unwrap();
        Ok(val * self.scale)
    }
}

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls /sys/bus/iio/devices/iio:device*/in_accel*")
        .output()
        .expect("Subprocess Error");
    let output = String::from_utf8_lossy(&output.stdout);
    let basedir = match output.lines().next() {
        Some(accel_sensor) => Path::new(accel_sensor).parent().unwrap(),
        None => panic!("Unable to find any accelerometer.")
    };
    println!("Accelerometer: {}", basedir.to_string_lossy());

    let mut accel = Accelerometer { base: basedir, scale: 0.0 };
    accel.init().expect("Cannot read accelerometer.");
    let mut dev_accel_x = accel.open("x").expect("IO Error.");
    let mut dev_accel_y = accel.open("y").expect("IO Error.");
    let mut dev_accel_z = accel.open("z").expect("IO Error.");
    let interval = std::time::Duration::from_secs(1);
    loop {
        let accel_x = accel.read(&mut dev_accel_x).unwrap();
        let accel_y = accel.read(&mut dev_accel_y).unwrap();
        let accel_z = accel.read(&mut dev_accel_z).unwrap();
        println!("g = ({}, {}, {})", accel_x, accel_y, accel_z);
        std::thread::sleep(interval);
    }
}
