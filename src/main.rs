// Copyright (c) 2024 Aleksandar Vasilić. All rights reserved.

use rand::Rng;

fn main() {
    // Read the text from the file
    let text = std::fs::read_to_string("./data/HDR21-22_Statistical_Annex_HDI_Table.txt")
        .expect("Unable to read file");

    // Parse the text into country name and HDI. They are seperated by a space.
    /* Filter into four categories
     Very high human development: >=0.800
     High human development: 0.700–0.799
     Medium human development: 0.550–0.699
     Low human development: =<0.550
    */
    let mut very_high_hdi = Vec::new();
    let mut high_hdi = Vec::new();
    let mut medium_hdi = Vec::new();
    let mut low_hdi = Vec::new();

    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let country = parts[0];
        let hdi = parts[1].parse::<f32>().expect("Unable to parse HDI");

        if hdi >= 0.800 {
            very_high_hdi.push(country);
        } else if hdi >= 0.700 {
            high_hdi.push(country);
        } else if hdi >= 0.550 {
            medium_hdi.push(country);
        } else {
            low_hdi.push(country);
        }
    }

    // Randomly pick 5 from each category
    let mut rng = rand::thread_rng();

    let mut very_high_hdi_sample = Vec::new();
    let mut high_hdi_sample = Vec::new();
    let mut medium_hdi_sample = Vec::new();
    let mut low_hdi_sample = Vec::new();

    for _ in 0..10 {
        very_high_hdi_sample.push(very_high_hdi.remove(rng.gen_range(0..very_high_hdi.len())));
        high_hdi_sample.push(high_hdi.remove(rng.gen_range(0..high_hdi.len())));
        medium_hdi_sample.push(medium_hdi.remove(rng.gen_range(0..medium_hdi.len())));
        low_hdi_sample.push(low_hdi.remove(rng.gen_range(0..low_hdi.len())));
    }

    // Print the sample
    println!("Very high human development: {:?}", very_high_hdi_sample);
    println!("High human development: {:?}", high_hdi_sample);
    println!("Medium human development: {:?}", medium_hdi_sample);
    println!("Low human development: {:?}", low_hdi_sample);
}
