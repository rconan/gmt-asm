use bincode::config;
use gmt_asm::Sys;
use nalgebra::Complex;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    time::Instant,
};

fn main() -> anyhow::Result<()> {
    let root = env::args()
        .skip(1)
        .next()
        .expect("expected 1 argument, found none");

    // let repo = env::var("DATA_REPO").unwrap();
    // let path = Path::new(&repo).join(format!("{}.tgz", root));

    let path = Path::new(&root);
    println!("Loading system from {:?}", path);
    let now = Instant::now();
    // let repo = env::var("DATA_REPO").expect("DATA_REPO not set");
    let file = File::open(path)?;
    let mut buffer = BufReader::new(file);
    let sys: Sys = bincode::serde::decode_from_std_read(&mut buffer, config::standard())?;
    // let sys = Sys::from_tarball(&path)?;
    println!("System loaded in {}s", now.elapsed().as_secs());

    let idx = [
        (21, vec![0]),
        (29, vec![0, 1]),
        (46, vec![0, 1, 2, 3]),
        (53, vec![0, 1]),
        (67, vec![0, 1]),
        (83, vec![0]),
        (89, vec![0]),
        (110, vec![0, 1]),
        (138, vec![0, 1]),
    ];
    let idx = [
        (142, vec![0]),
        (166, vec![0, 1]),
        (194, vec![0, 1]),
        (223, vec![0, 1]),
        (242, vec![0]),
        (327, vec![0, 1]),
        (377, vec![0, 1]),
        (486, vec![0, 1]),
    ];
    // println!("Computing the system singular values");
    let now = Instant::now();
    let nu = sys.frequencies();
    let mut lsms = vec![];
    for (i, s_i) in idx.into_iter() {
        let (u, s) = sys[i].left_singular_modes();
        // dbg!(&s[..3]);
        for j in s_i {
            let u0 = u.column(j);
            // let u0_nrm: Vec<_> = u0.as_slice().iter().map(|x| x.norm()).collect();
            lsms.push(LeftSingularMode {
                nu: nu[i],
                s: s[j],
                m: u0.as_slice().to_vec(),
            });
        }
    }
    // dbg!(&u0.as_slice()[..10]);
    println!("Singular modes computed in {}s", now.elapsed().as_secs());

    // let path = Path::new(&repo).join(format!("{}_sv.pkl", root));
    let file = File::create(path.with_extension("lsm.pkl"))?;
    let mut buffer = BufWriter::new(file);
    serde_pickle::to_writer(&mut buffer, &lsms, Default::default())?;

    Ok(())
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LeftSingularMode {
    nu: f64,
    s: f64,
    m: Vec<Complex<f64>>,
}
