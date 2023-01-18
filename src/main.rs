use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
        let mut music = vec!["b flat major scale", "g harmonic minor scale", "g melodic minor scale", "e flat major scale", "c harmonic minor scale", "c melodic minor scale", "a flat major scale", "f harmonic minor scale", "f melodic minor scale", "f sharp major scale", "d sharp harmonic minor scale", "d sharp melodic minor scale", "blues scale scale", "chromatic scale scale", "whole tone on c", "whole tone on c sharp", "a major in thirds", "e flat major in thirds", "b flat major arpeggio", "g minor arpeggio", "dominant 7th of b flat", "e flat major arpeggio", "c minor arpeggio", "dominant 7th of e flat", "a flat major arpeggio", "f minor arpeggio", "dominant 7th of a flat", "f sharp major arpeggio", "d sharp minor arpeggio", "dominant 7th of f sharp", "c major broken chord", "a minor broken chord", "f major broken chord", "d minor broken cord", "sustained scale"];
        
        loop {
            
            let mut nothing = String::new();

            io::stdin()
                .read_line(&mut nothing)
                .expect("Failed to read line");
            if music.len() < 1 {
                println!("You have no scales to practice");
                break;
            }
            if music.len() < 5 {
                println!("less than 5 scales left");
            }
            let scale = music.choose(&mut thread_rng()).unwrap().to_string();
            println!("{}", scale);
            music.remove(music.iter().position(|x| *x == scale).expect("not found"));

            
        }
}




