use tlsn_core::{attestation::Attestation, presentation::Presentation, CryptoProvider, Secrets};
use tlsn_formats::http::HttpTranscript;

// Presentation logic draws from the examples in the tlsn crate
// https://github.com/tlsnotary/tlsn/tree/main/crates/examples

pub fn build_presentation() {
    let attestation_bytes = std::fs::read("vcnotary.attestation.tlsn").unwrap();
    let attestation: Attestation = bincode::deserialize(&attestation_bytes).unwrap();

    let secrets_bytes = std::fs::read("vcnotary.secrets.tlsn").unwrap();
    let secrets: Secrets = bincode::deserialize(&secrets_bytes).unwrap();

    let transcript = HttpTranscript::parse(secrets.transcript()).unwrap();

    let mut builder = secrets.transcript_proof_builder();

    let request = &transcript.requests[0];

    // Hide bearer token
    builder.reveal_sent(&request.without_data()).unwrap();
    builder.reveal_sent(&request.request.target).unwrap();
    for header in &request.headers {
        if !header.name.as_str().eq_ignore_ascii_case("authorization") {
            builder.reveal_sent(header).unwrap();
        } else {
            builder.reveal_sent(&header.without_value()).unwrap();
        }
    }

    let response = &transcript.responses[0];

    builder.reveal_recv(response).unwrap();

    let transcript_proof = builder.build().unwrap();

    let provider = CryptoProvider::default();

    let mut builder = attestation.presentation_builder(&provider);

    builder
        .identity_proof(secrets.identity_proof())
        .transcript_proof(transcript_proof);

    let presentation: Presentation = builder.build().unwrap();

    std::fs::write(
        "vcnotary.presentation.tlsn",
        bincode::serialize(&presentation).unwrap(),
    ).unwrap();
}
