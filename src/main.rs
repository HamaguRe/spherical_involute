//! 球面インボリュート曲線を計算する

use std::fs;
use std::io::{Write, BufWriter};
use quaternion_core as quat;

const PI: f64 = std::f64::consts::PI;

// --- 基礎円錐諸元 --- //
const GENERATRIX: f64 = 20.0;  // 基礎円錐の母線長 [mm]
const PHI: f64 = 60.0 * (PI / 180.0);  // 基礎円錐角 [rad]
const PSI: f64 = 0.5 * PI - PHI;

// ----- 計算設定 ----- //
const N_THETA: usize = 20;
const DELTA_THETA: f64 = 0.1;  // [rad]

fn main() {
    // CSVファイルにデータ保存（同一ファイルが存在したら上書き）
    let mut base_cone_surface = BufWriter::new( fs::File::create("./base_cone_surface.csv").unwrap() );
    let mut tooth_surface = BufWriter::new( fs::File::create("./tooth_surface.csv").unwrap() );
    base_cone_surface.write(b"x,y,z\n").unwrap();
    tooth_surface.write(b"x,y,z\n").unwrap();

    // 基礎円錐を描画するための座標計算
    calc_base_cone(&mut base_cone_surface, 10, 50);

    for i in 0..N_THETA {
        let s = spherical_inv(DELTA_THETA * i as f64);

        // データ書き出し
        tooth_surface.write( format!("{:.4},{:.4},{:.4}\n", s[0], s[1], s[2]).as_bytes() ).unwrap();
    }
}

/// 球面インボリュート
/// 
/// * theta: 基礎円錐上を転がる仮想円盤の、初期位置からの回転角 [rad]
fn spherical_inv(theta: f64) -> quat::Vector3<f64> {
    let tau = PHI.sin() * theta;
    let a = [GENERATRIX * PHI.sin(), 0.0, -GENERATRIX * PHI.cos()];
    let c = [PSI.sin(), 0.0, PSI.cos()];

    let q_z = quat::from_axis_angle([0.0, 0.0, 1.0], theta);
    let q_c = quat::from_axis_angle(c, -tau);

    let tmp = quat::mul(q_z, q_c);
    quat::point_rotation(tmp, a)
}

/// 基礎円錐を計算してファイルに書き出す
fn calc_base_cone(f: &mut BufWriter<fs::File>, gen_split: usize, theta_split: usize) {
    for i in 0..(gen_split + 1) {
        let gen = (GENERATRIX / gen_split as f64) * (i as f64);
        let a = [gen * PHI.sin(), 0.0, -gen * PHI.cos()];
        for j in 0..(theta_split + 1) {
            let theta = (2.0 * PI / theta_split as f64) * j as f64;

            let q = quat::from_axis_angle([0.0, 0.0, 1.0], theta);
            let b = quat::point_rotation(q, a);

            f.write( format!("{:.4},{:.4},{:.4}\n", b[0], b[1], b[2]).as_bytes() ).unwrap();
        }
    }
}