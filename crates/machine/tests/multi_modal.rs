use machine::Fragment;
use machine::completion::encode;
use machine::fragment::DataSource;
use rig::completion::Message;
use rig::completion::message::UserContent;

fn image_url() -> Fragment {
    Fragment::image(DataSource::Url("https://example.com/img.png".into()), None)
}

fn image_base64() -> Fragment {
    Fragment::image(DataSource::Base64("SGVsbG8=".into()), None)
}

fn image_raw() -> Fragment {
    Fragment::image(DataSource::Raw(vec![0x89, 0x50, 0x4e, 0x47]), None)
}

fn image_string() -> Fragment {
    Fragment::image(DataSource::String("data:image/png;base64,abc".into()), None)
}

fn audio_url() -> Fragment {
    Fragment::audio(
        DataSource::Url("https://example.com/audio.mp3".into()),
        None,
    )
}

fn audio_base64() -> Fragment {
    Fragment::audio(DataSource::Base64("AAAA".into()), None)
}

fn audio_raw() -> Fragment {
    Fragment::audio(DataSource::Raw(vec![0xff, 0xfb, 0x90, 0x00]), None)
}

fn document_base64() -> Fragment {
    Fragment::document(
        DataSource::Base64("JVBERi0xLjQK".into()),
        Some("application/pdf".into()),
    )
}

fn video_url() -> Fragment {
    Fragment::video(DataSource::Url("https://example.com/vid.mp4".into()), None)
}

/// Assert that encode returns Some(Message::User { content }) and that the
/// content contains exactly one item matching the expected UserContent variant.
fn assert_user_content(frag: &Fragment, kind: &str) {
    let msg = encode(frag, false).expect("multi-modal fragment encodes");
    let Message::User { content } = &msg else {
        panic!("expected Message::User for {kind}, got {:?}", msg);
    };
    assert_eq!(
        content.len(),
        1,
        "content should have exactly 1 item for {kind}"
    );
    let actual = content.iter().next().unwrap();
    let variant = match actual {
        UserContent::Image(_) => "image",
        UserContent::Audio(_) => "audio",
        UserContent::Video(_) => "video",
        UserContent::Document(_) => "document",
        _ => "other",
    };
    assert_eq!(variant, kind, "expected {kind} content, got {variant}");
}

#[test]
fn encode_image_url() {
    assert_user_content(&image_url(), "image");
}

#[test]
fn encode_image_base64() {
    assert_user_content(&image_base64(), "image");
}

#[test]
fn encode_image_raw() {
    assert_user_content(&image_raw(), "image");
}

#[test]
fn encode_image_string() {
    assert_user_content(&image_string(), "image");
}

#[test]
fn encode_audio_url() {
    assert_user_content(&audio_url(), "image"); // Url always maps to image
}

#[test]
fn encode_audio_base64() {
    assert_user_content(&audio_base64(), "audio");
}

#[test]
fn encode_audio_raw() {
    assert_user_content(&audio_raw(), "audio");
}

#[test]
fn encode_document_base64() {
    assert_user_content(&document_base64(), "document");
}

#[test]
fn encode_video_url() {
    assert_user_content(&video_url(), "image"); // Url always maps to image
}

#[test]
fn content_as_text_returns_placeholder() {
    assert_eq!(image_url().content_as_text(), "<image>");
    assert_eq!(audio_url().content_as_text(), "<audio>");
    assert_eq!(video_url().content_as_text(), "<video>");
    assert_eq!(document_base64().content_as_text(), "<document>");
}

#[test]
fn encode_text_fallback_unchanged() {
    let frag = Fragment::user("hello");
    let msg = encode(&frag, false).expect("user text encodes");
    let Message::User { content } = &msg else {
        panic!("expected Message::User");
    };
    assert!(matches!(
        content.iter().next().unwrap(),
        UserContent::Text(_)
    ));
}
