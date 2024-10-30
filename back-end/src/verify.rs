use tlsn_core::{
    presentation::{Presentation, PresentationOutput},
    CryptoProvider,
};

pub fn verify_presentation() {
    let presentation_bytes = std::fs::read("vcnotary.presentation.tlsn").unwrap();
    let presentation: Presentation = bincode::deserialize(&presentation_bytes).unwrap();

    let provider = CryptoProvider::default();

    let PresentationOutput { transcript, .. } = presentation.verify(&provider).unwrap();

    let mut partial_transcript = transcript.unwrap();

    partial_transcript.set_unauthed(b'X');

    let sent = String::from_utf8_lossy(partial_transcript.sent_unsafe());
    let recv = String::from_utf8_lossy(partial_transcript.received_unsafe());

    println!("-------------------------------------------------------------------");
    println!("Data sent:\n");
    println!("{}\n", sent);
    println!("Data received:\n");
    println!("{}\n", recv);
    println!("-------------------------------------------------------------------");
}
