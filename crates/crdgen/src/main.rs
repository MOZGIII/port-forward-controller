//! Prints CRDs.

use kube::CustomResourceExt;

fn main() {
    print!("{}", serde_yaml::to_string(&crd::PCPMap::crd()).unwrap())
}
