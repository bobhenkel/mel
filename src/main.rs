use rust_embed::RustEmbed;

//Used to write embedded files out of binary
use std::fs::File;
use std::io::prelude::*;

//Used to decompress files
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(RustEmbed)]
#[folder = "payload/"]
struct Asset;

fn main() -> std::io::Result<()> {
    let goss_tar_gz = Asset::get("linux-amd64/goss.tar.gz").unwrap();
    //println!("{:?}", goss.as_ref());
    let helm = Asset::get("linux-amd64/helm-v3.1.1-linux-amd64.tar.gz").unwrap();
    let kubectl = Asset::get("linux-amd64/kubectl.tar.gz").unwrap();
    let packer = Asset::get("linux-amd64/packer_1.5.4_linux_amd64.zip").unwrap();
    let terraform = Asset::get("linux-amd64/terraform_0.12.21_linux_amd64.zip").unwrap();

    let mut file = File::create("goss.tar.gz")?;
    file.write_all(&goss_tar_gz)?;
    // for file in Asset::iter() {
    //     println!("{}", file.as_ref());
    // }


    decompress("/home/bob/CLionProjects/mel/payload/linux-amd64/goss.tar.gz");

    Ok(())
}

fn decompress(path: &str ) -> std::io::Result<()> {
    let path = path;
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}
