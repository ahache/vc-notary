use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use tlsn_common::config::ProtocolConfig;
use tlsn_core::{request::RequestConfig, transcript::TranscriptCommitConfig};
use tlsn_prover::{Prover, ProverConfig};
use tlsn_formats::http::{DefaultHttpCommitter, HttpCommit, HttpTranscript};

use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, Request, StatusCode};
use hyper_util::rt::TokioIo;
use std::{env, str};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use tracing::debug;
use utils::range::RangeSet;

const MAX_SENT_DATA: usize = 1 << 12;
const MAX_RECV_DATA: usize = 1 << 14;

pub async fn notarize_api_data(access_token: String) {
    let notary_client = NotaryClient::builder()
        .host("127.0.0.1")
        .port(7047)
        .enable_tls(false)
        .build()
        .unwrap();

    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()
        .unwrap();

    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .expect("Could not connect to notary. Make sure it is running.");

    let protocol_config = ProtocolConfig::builder()
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()
        .unwrap();

    let prover_config = ProverConfig::builder()
        .server_name("reddit.com")
        .protocol_config(protocol_config)
        .build()
        .unwrap();

    let prover = Prover::new(prover_config)
        .setup(notary_connection.compat())
        .await
        .unwrap();

    let client_socket = tokio::net::TcpStream::connect(("reddit.com", 443))
        .await
        .unwrap();

    let (tls_connection, prover_fut) = prover.connect(client_socket.compat()).await.unwrap();

    let prover_task = tokio::spawn(prover_fut);

    let (mut request_sender, connection) =
        hyper::client::conn::http1::handshake(TokioIo::new(tls_connection.compat()))
            .await
            .unwrap();

    tokio::spawn(connection);

    let request = Request::builder()
        .uri(format!("https://oauth.reddit.com/subreddits/mine/moderator"))
        .header("Host", "oauth.reddit.com")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "identity")
        .header("User-Agent", "VC Notary")
        .header("Authorization", format!("Bearer {}", &access_token))
        .header("Connection", "close")
        .body(Empty::<Bytes>::new())
        .unwrap();

    request_sender.send_request(request).await.unwrap();

    let prover = prover_task.await.unwrap().unwrap();

    let mut prover = prover.start_notarize();

    let transcript = HttpTranscript::parse(prover.transcript()).unwrap();

    let mut builder = TranscriptCommitConfig::builder(prover.transcript());

    DefaultHttpCommitter::default().commit_transcript(&mut builder, &transcript).unwrap();

    let config = builder.build().unwrap();

    prover.transcript_commit(config);

    let request_config = RequestConfig::default();
    let (attestation, secrets) = prover.finalize(&request_config).await.unwrap();

    tokio::fs::write(
        "vcnotary.attestation.tlsn",
        bincode::serialize(&attestation).unwrap(),
    )
    .await
    .unwrap();

    tokio::fs::write(
        "vcnotary.secrets.tlsn",
        bincode::serialize(&secrets).unwrap(),
    )
    .await
    .unwrap();
}
