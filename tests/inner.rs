use audiotags::*;
use id3::TagLike;
use std::fs;
use std::path::Path;
use tempfile::Builder;

#[test]
fn test_inner() {
    let tmp = Builder::new().suffix(".mp3").tempfile().unwrap();
    fs::copy("assets/a.mp3", &tmp).unwrap();

    let tmp_path = tmp.path();

    let mut innertag = metaflac::Tag::default();
    let title = "title from metaflac::Tag";
    let artist = "Billy Foo";
    let album_artist = "Billy Foo & The Bars";
    innertag.vorbis_comments_mut().set_title(vec![title]);
    innertag.vorbis_comments_mut().set_artist(vec![artist]);
    innertag
        .vorbis_comments_mut()
        .set_album_artist(vec![album_artist]);

    let tag: FlacTag = innertag.into();
    let mut id3tag = tag.to_dyn_tag(TagType::Id3v2);

    id3tag
        .write_to_path(Path::new(tmp_path.to_str().unwrap()))
        .expect("Fail to write!");

    let id3tag_reload = Tag::default()
        .read_from_path(tmp_path)
        .expect("Fail to read!");

    assert_eq!(id3tag_reload.title(), Some(title));
    assert_eq!(id3tag_reload.artist(), Some(artist));
    assert_eq!(id3tag_reload.album_artist(), Some(album_artist));

    // let id3tag: Id3v2Tag = id3tag_reload.into();
    let mut id3tag_inner: id3::Tag = id3tag_reload.into();
    let timestamp = id3::Timestamp {
        year: 2013,
        month: Some(2u8),
        day: Some(5u8),
        hour: Some(6u8),
        minute: None,
        second: None,
    };

    id3tag_inner.set_date_recorded(timestamp);
    id3tag_inner
        .write_to_path(tmp_path, id3::Version::Id3v24)
        .expect("Fail to write!");

    let id3tag_reload = id3::Tag::read_from_path(tmp_path).expect("Fail to read!");
    assert_eq!(id3tag_reload.date_recorded(), Some(timestamp));
    assert_eq!(id3tag_reload.artist(), Some(artist));
    assert_eq!(id3tag_reload.album_artist(), Some(album_artist));
}
