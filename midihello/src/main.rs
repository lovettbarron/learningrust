fn main() {
    println!("Hello C3, {}!", note_to_freq(48.0));
    println!("Hello A4, {}!", note_to_freq(69.0));
}

fn note_to_freq(note: f32) -> f32 {
    //    return (a / 32) * pow(2, ((note - 9) / 12.0)); c
    let base: f32 = 2.0; // for pow
    let notediv: f32 = (note - 9.0) / 12.0;
    const a: f32 = 440.00; //freq of A

    (a / 32.0) * base.powf(notediv)
}
