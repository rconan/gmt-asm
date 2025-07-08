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

    let idx = [21, 29, 46, 53, 67];
    // println!("Computing the system singular values");
    // let now = Instant::now();
    let nu = sys.frequencies();
    let mut lsms = vec![];
    for i in idx.into_iter() {
        let (u, s) = sys[i].left_singular_modes();
        // dbg!(&s[..3]);
        let u0 = u.column(0);
        // let u0_nrm: Vec<_> = u0.as_slice().iter().map(|x| x.norm()).collect();
        lsms.push(LeftSingularMode {
            nu: nu[i],
            s: s[0],
            m: u0.as_slice().to_vec(),
        });
    }
    // dbg!(&u0.as_slice()[..10]);
    // println!("Singular modes computed in {}s", now.elapsed().as_secs());

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
