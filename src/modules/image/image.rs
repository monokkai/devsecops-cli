use crate::cli::{ImageAction, ImageArgs};
use colored::*;
use exif::{Field, In, Tag};
use std::fs::File;
use std::io::BufReader;

pub fn handle(args: ImageArgs) {
    match args.action {
        ImageAction::Extract { path, all } => {
            if let Err(e) = extract_image_metadata(&path, all) {
                eprintln!("{}: {}", "Error".red(), e);
            }
        }
    }
}

fn extract_image_metadata(path: &str, show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(_) => {
            println!("{}: No EXIF data found in image", "Warning".yellow());
            return Ok(());
        }
    };

    println!("{}", "Extracted metadata:".green().bold());

    let interesting_tags = if show_all {
        // Show all tags if --all flag is set
        exif.fields().collect::<Vec<_>>()
    } else {
        // Only show common interesting tags
        exif.fields()
            .filter(|field| {
                matches!(
                    field.tag,
                    Tag::DateTime
                        | Tag::DateTimeOriginal
                        | Tag::DateTimeDigitized
                        | Tag::GPSLatitude
                        | Tag::GPSLongitude
                        | Tag::GPSLatitudeRef
                        | Tag::GPSLongitudeRef
                        | Tag::Make
                        | Tag::Model
                        | Tag::Software
                        | Tag::Artist
                        | Tag::Copyright
                        | Tag::ImageDescription
                )
            })
            .collect()
    };

    for field in interesting_tags {
        let tag_name = format!("{:?}", field.tag);
        let value = match field.tag {
            Tag::GPSLatitude | Tag::GPSLongitude => {
                if let Ok(coord) = field.value.get_float(0) {
                    format!("{}°", coord)
                } else {
                    field.display_value().to_string()
                }
            }
            _ => field.display_value().to_string(),
        };

        println!("{:20}: {}", tag_name.cyan(), value);
    }

    // Try to extract GPS coordinates if available
    if let (Some(lat), Some(lon)) = get_gps_coordinates(&exif) {
        println!();
        println!("{}", "GPS Coordinates:".green().bold());
        println!("Latitude:  {}", lat.to_string().yellow());
        println!("Longitude: {}", lon.to_string().yellow());
        println!();
        println!(
            "{}: https://www.google.com/maps/place/{},{}",
            "View on map".blue(),
            lat,
            lon
        );
    }

    Ok(())
}

fn get_gps_coordinates(exif: &exif::Exif) -> (Option<f64>, Option<f64>) {
    let lat = get_gps_coordinate(exif, Tag::GPSLatitude, Tag::GPSLatitudeRef);
    let lon = get_gps_coordinate(exif, Tag::GPSLongitude, Tag::GPSLongitudeRef);
    (lat, lon)
}

fn get_gps_coordinate(exif: &exif::Exif, coord_tag: Tag, ref_tag: Tag) -> Option<f64> {
    let coord = exif.get_field(coord_tag, In::PRIMARY)?;
    let ref_val = exif.get_field(ref_tag, In::PRIMARY)?;

    let coord_vec = coord.value.as_rational()?.get(0..3)?;
    let degrees = coord_vec[0].to_f64();
    let minutes = coord_vec[1].to_f64();
    let seconds = coord_vec[2].to_f64();

    let mut coord = degrees + minutes / 60.0 + seconds / 3600.0;

    if ref_val.display_value().to_string() == "S" || ref_val.display_value().to_string() == "W" {
        coord = -coord;
    }

    Some(coord)
}
