use std::{fs, io::Read, os::unix::fs::OpenOptionsExt, path::Path, str::FromStr};
use colored::Colorize;
use flate2::read::GzDecoder;
use oci_distribution::{
    client::{ClientConfig, ClientProtocol},
    manifest::{IMAGE_LAYER_GZIP_MEDIA_TYPE, IMAGE_MANIFEST_MEDIA_TYPE},
    secrets::RegistryAuth,
    Client, Reference, RegistryOperation,
};
use tar::{Archive, Builder, Header};

use crate::clean::cleanup;

static MEDIA_TYPE_ROOTFS_GZIP: &str = "application/vnd.docker.image.rootfs.diff.tar.gzip";

pub async fn fetch_image(image: &str, registry: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "Fetching image {} from registry {}",
        image.blue().bold(),
        registry.blue().bold()
    );

    let client = Client::default();
    let image_reference = Reference::from_str(image).unwrap();

    let image = client
        .pull(
            &image_reference,
            &RegistryAuth::Anonymous,
            vec![MEDIA_TYPE_ROOTFS_GZIP, IMAGE_LAYER_GZIP_MEDIA_TYPE],
        )
        .await?;
    println!("Image pulled.");
    println!("Processing {} layers...", image.layers.len());

    // Create temporary directory
    let temp_dir = "/tmp/dockerdump".to_string(); // Use process ID for uniqueness
    cleanup(&temp_dir)?;
    // Create the base directory
    fs::create_dir_all(&temp_dir)?;

    // Process each layer
    for (i, layer) in image.layers.into_iter().enumerate() {
        println!("Processing layer {}", i + 1);
        let layer_dir = format!("{}/layer_{}", temp_dir, i + 1);
        fs::create_dir_all(&layer_dir)?;

        // Decompress gzip
        let mut gz = GzDecoder::new(&layer.data[..]);
        let mut decompressed = Vec::new();
        gz.read_to_end(&mut decompressed)?;

        // Custom extraction to handle permissions
        let mut archive = Archive::new(&decompressed[..]);
        
        // Disable all permission preservation
        archive.set_preserve_permissions(false);
        archive.set_preserve_mtime(false);
        archive.set_preserve_ownerships(false);
        archive.set_unpack_xattrs(false);
        
        // Process each entry with modified permissions
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let target = Path::new(&layer_dir).join(path.strip_prefix("/").unwrap_or(&path));

            // Create parent directories if needed
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            // Set permissive mode in the header
            let mut header = entry.header().clone();
            header.set_mode(0o666); // rw-rw-rw-
            
            if header.entry_type().is_dir() {
                fs::create_dir_all(&target)?;
            } else {
                // Create file and copy contents
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .mode(0o666) // Set mode during creation
                    .open(&target)?;
                std::io::copy(&mut entry, &mut file)?;
            }
        }
    }
    Ok(temp_dir)
}