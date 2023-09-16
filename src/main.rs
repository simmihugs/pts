mod pts_loader;

use pts_loader::dataset::DataSet;

fn main() -> std::io::Result<()> {
    let filename = "hdplus_20230915_26886.pts";
    match DataSet::init(filename) {
    	Ok (dataset) => dataset.print_n_si(3),
    	Err (e) => println!("Some error: {}", e),
    }

    Ok(())
}
