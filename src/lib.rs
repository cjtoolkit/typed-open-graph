#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use humantime::format_rfc3339;
use std::fmt::Display;
use std::time::SystemTime;

const PREFIX: &str = "og: https://ogp.me/ns#";

#[allow(dead_code)]
#[derive(Debug)]
enum OpenGraphDataTypes {
    Boolean(bool),
    String(String),
    Float(f64),
    Integer(isize),
    Url(url::Url),
}

impl Display for OpenGraphDataTypes {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Boolean(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Self::String(s) => s.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Integer(i) => i.to_string(),
            Self::Url(url) => url.to_string(),
        };
        write!(f1, "{}", str)
    }
}

/// The determiner of the object being tagged.
pub enum Determiner {
    A,
    An,
    The,
    Blank,
    Auto,
}

impl Display for Determiner {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::A => "a",
            Self::An => "an",
            Self::The => "the",
            Self::Blank => "",
            Self::Auto => "auto",
        };
        write!(f1, "{}", str)
    }
}

/// The status of the payment.
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Expired,
}

impl Display for PaymentStatus {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Pending => "PENDING",
            Self::Paid => "PAID",
            Self::Failed => "FAILED",
            Self::Expired => "EXPIRED",
        };
        write!(f1, "{}", str)
    }
}

/// Profile Gender
pub enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Male => "male",
            Self::Female => "female",
        };
        write!(f1, "{}", str)
    }
}

pub trait OpenGraphTypeMarker: Send + Sync + Sized {
    const TYPE: &'static str;
    const EXTRA_PREFIX: &'static str;

    fn builder(
        title: impl Into<&'static str>,
        image: url::Url,
        url: url::Url,
    ) -> OpenGraphBuilder<Self> {
        OpenGraphBuilder::new(title, image, url)
    }
}

#[derive(Debug)]
pub struct MusicSong;

impl OpenGraphTypeMarker for MusicSong {
    const TYPE: &'static str = "music.song";
    const EXTRA_PREFIX: &'static str = "music: https://ogp.me/ns/music#";
}

#[derive(Debug)]
pub struct MusicAlbum;

impl OpenGraphTypeMarker for MusicAlbum {
    const TYPE: &'static str = "music.album";
    const EXTRA_PREFIX: &'static str = "music: https://ogp.me/ns/music#";
}

#[derive(Debug)]
pub struct MusicPlaylist;

impl OpenGraphTypeMarker for MusicPlaylist {
    const TYPE: &'static str = "music.playlist";
    const EXTRA_PREFIX: &'static str = "music: https://ogp.me/ns/music#";
}

#[derive(Debug)]
pub struct MusicRadioStation;

impl OpenGraphTypeMarker for MusicRadioStation {
    const TYPE: &'static str = "music.radio_station";
    const EXTRA_PREFIX: &'static str = "music: https://ogp.me/ns/music#";
}

#[derive(Debug)]
pub struct VideoMovie;

impl OpenGraphTypeMarker for VideoMovie {
    const TYPE: &'static str = "video.movie";
    const EXTRA_PREFIX: &'static str = "video: https://ogp.me/ns/video#";
}

#[derive(Debug)]
pub struct VideoEpisode;

impl OpenGraphTypeMarker for VideoEpisode {
    const TYPE: &'static str = "video.episode";
    const EXTRA_PREFIX: &'static str = "video: https://ogp.me/ns/video#";
}

#[derive(Debug)]
pub struct VideoTvShow;

impl OpenGraphTypeMarker for VideoTvShow {
    const TYPE: &'static str = "video.tv_show";
    const EXTRA_PREFIX: &'static str = "video: https://ogp.me/ns/video#";
}

#[derive(Debug)]
pub struct VideoOther;

impl OpenGraphTypeMarker for VideoOther {
    const TYPE: &'static str = "video.other";
    const EXTRA_PREFIX: &'static str = "video: https://ogp.me/ns/video#";
}

#[derive(Debug)]
pub struct Article;

impl OpenGraphTypeMarker for Article {
    const TYPE: &'static str = "article";
    const EXTRA_PREFIX: &'static str = "article: https://ogp.me/ns/article#";
}

#[derive(Debug)]
pub struct Book;

impl OpenGraphTypeMarker for Book {
    const TYPE: &'static str = "book";
    const EXTRA_PREFIX: &'static str = "book: https://ogp.me/ns/book#";
}

#[derive(Debug)]
pub struct PaymentLink;

impl OpenGraphTypeMarker for PaymentLink {
    const TYPE: &'static str = "payment.link";
    const EXTRA_PREFIX: &'static str = "payment: https://ogp.me/ns/payment#";
}

#[derive(Debug)]
pub struct Profile;

impl OpenGraphTypeMarker for Profile {
    const TYPE: &'static str = "profile";
    const EXTRA_PREFIX: &'static str = "profile: https://ogp.me/ns/profile#";
}

#[derive(Debug)]
pub struct Website;

impl OpenGraphTypeMarker for Website {
    const TYPE: &'static str = "website";
    const EXTRA_PREFIX: &'static str = "website: https://ogp.me/ns/website#";
}

#[derive(Debug)]
pub struct OpenGraphBuilder<M: OpenGraphTypeMarker> {
    list: Vec<(&'static str, OpenGraphDataTypes)>,
    marker: std::marker::PhantomData<M>,
}

impl<M: OpenGraphTypeMarker> OpenGraphBuilder<M> {
    pub fn new(title: impl Into<&'static str>, image: url::Url, url: url::Url) -> Self {
        Self {
            list: vec![
                (
                    "og:title",
                    OpenGraphDataTypes::String(title.into().to_string()),
                ),
                ("og:image", OpenGraphDataTypes::Url(image)),
                ("og:url", OpenGraphDataTypes::Url(url)),
            ],
            marker: std::marker::PhantomData,
        }
    }

    pub fn image_secure_url(mut self, image_secure_url: url::Url) -> Self {
        self.list.push((
            "og:image:secure_url",
            OpenGraphDataTypes::Url(image_secure_url),
        ));
        self
    }

    pub fn image_type(mut self, image_type: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:image:type",
            OpenGraphDataTypes::String(image_type.into().to_string()),
        ));
        self
    }

    pub fn image_width(mut self, image_width: isize) -> Self {
        self.list
            .push(("og:image:width", OpenGraphDataTypes::Integer(image_width)));
        self
    }

    pub fn image_height(mut self, image_height: isize) -> Self {
        self.list
            .push(("og:image:height", OpenGraphDataTypes::Integer(image_height)));
        self
    }

    pub fn image_alt(mut self, image_alt: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:image:alt",
            OpenGraphDataTypes::String(image_alt.into().to_string()),
        ));
        self
    }

    pub fn audio(mut self, audio: url::Url) -> Self {
        self.list.push(("og:audio", OpenGraphDataTypes::Url(audio)));
        self
    }

    pub fn audio_secure_url(mut self, audio_secure_url: url::Url) -> Self {
        self.list.push((
            "og:audio:secure_url",
            OpenGraphDataTypes::Url(audio_secure_url),
        ));
        self
    }

    pub fn audio_type(mut self, audio_type: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:audio:type",
            OpenGraphDataTypes::String(audio_type.into().to_string()),
        ));
        self
    }

    pub fn description(mut self, description: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:description",
            OpenGraphDataTypes::String(description.into().to_string()),
        ));
        self
    }

    pub fn determiner(mut self, determiner: Determiner) -> Self {
        self.list.push((
            "og:determiner",
            OpenGraphDataTypes::String(determiner.to_string()),
        ));
        self
    }

    pub fn locale(mut self, locale: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:locale",
            OpenGraphDataTypes::String(locale.into().to_string()),
        ));
        self
    }

    pub fn locale_alternate(mut self, locales: Vec<impl Into<&'static str>>) -> Self {
        for locale in locales {
            self.list.push((
                "og:locale:alternate",
                OpenGraphDataTypes::String(locale.into().to_string()),
            ));
        }
        self
    }

    pub fn site_name(mut self, site_name: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:site_name",
            OpenGraphDataTypes::String(site_name.into().to_string()),
        ));
        self
    }

    pub fn video(mut self, video: url::Url) -> Self {
        self.list.push(("og:video", OpenGraphDataTypes::Url(video)));
        self
    }

    pub fn video_secure_url(mut self, video_secure_url: url::Url) -> Self {
        self.list.push((
            "og:video:secure_url",
            OpenGraphDataTypes::Url(video_secure_url),
        ));
        self
    }

    pub fn video_type(mut self, video_type: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:video:type",
            OpenGraphDataTypes::String(video_type.into().to_string()),
        ));
        self
    }

    pub fn video_width(mut self, video_width: isize) -> Self {
        self.list
            .push(("og:video:width", OpenGraphDataTypes::Integer(video_width)));
        self
    }

    pub fn video_height(mut self, video_height: isize) -> Self {
        self.list
            .push(("og:video:height", OpenGraphDataTypes::Integer(video_height)));
        self
    }

    pub fn video_alt(mut self, video_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "og:video:alt",
            OpenGraphDataTypes::String(video_tag.into().to_string()),
        ));
        self
    }

    pub fn custom(mut self, key: impl Into<&'static str>, value: impl Into<&'static str>) -> Self {
        self.list.push((
            key.into(),
            OpenGraphDataTypes::String(value.into().to_string()),
        ));
        self
    }

    pub fn build(self) -> OpenGraph {
        OpenGraph {
            prefix: format!("{} {}", PREFIX, M::EXTRA_PREFIX),
            og_type: M::TYPE.to_string(),
            list: self
                .list
                .iter()
                .map(|i| (i.0.to_string(), i.1.to_string()))
                .collect(),
        }
    }
}

impl<M: OpenGraphTypeMarker> Into<OpenGraph> for OpenGraphBuilder<M> {
    fn into(self) -> OpenGraph {
        self.build()
    }
}

impl OpenGraphBuilder<MusicSong> {
    pub fn music_duration(mut self, music_duration: isize) -> Self {
        if music_duration >= 1 {
            self.list.push((
                "music:duration",
                OpenGraphDataTypes::Integer(music_duration),
            ));
        }
        self
    }

    pub fn music_album(mut self, music_album: url::Url) -> Self {
        self.list
            .push(("music:album", OpenGraphDataTypes::Url(music_album)));
        self
    }

    pub fn music_album_disc(mut self, music_album_disc: isize) -> Self {
        if music_album_disc >= 1 {
            self.list.push((
                "music:album:disc",
                OpenGraphDataTypes::Integer(music_album_disc),
            ));
        }
        self
    }

    pub fn music_album_track(mut self, music_album_track: isize) -> Self {
        if music_album_track >= 1 {
            self.list.push((
                "music:album:track",
                OpenGraphDataTypes::Integer(music_album_track),
            ));
        }
        self
    }

    pub fn music_musician(mut self, music_musician: url::Url) -> Self {
        self.list
            .push(("music:musician", OpenGraphDataTypes::Url(music_musician)));
        self
    }
}

impl OpenGraphBuilder<MusicAlbum> {
    pub fn music_song(mut self, music_song: url::Url) -> Self {
        self.list
            .push(("music:song", OpenGraphDataTypes::Url(music_song)));
        self
    }

    pub fn music_song_disc(mut self, music_song_disc: isize) -> Self {
        if music_song_disc >= 1 {
            self.list.push((
                "music:song:disc",
                OpenGraphDataTypes::Integer(music_song_disc),
            ));
        }
        self
    }

    pub fn music_song_track(mut self, music_song_track: isize) -> Self {
        if music_song_track >= 1 {
            self.list.push((
                "music:song:track",
                OpenGraphDataTypes::Integer(music_song_track),
            ));
        }
        self
    }

    pub fn music_musician(mut self, music_musician: url::Url) -> Self {
        self.list
            .push(("music:musician", OpenGraphDataTypes::Url(music_musician)));
        self
    }

    pub fn music_release_date(mut self, music_release_date: SystemTime) -> Self {
        self.list.push((
            "music:release_date",
            OpenGraphDataTypes::String(format_rfc3339(music_release_date).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn music_release_date_chrono(
        mut self,
        music_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "music:release_date",
            OpenGraphDataTypes::String(music_release_date.into().to_rfc3339()),
        ));
        self
    }
}

impl OpenGraphBuilder<MusicPlaylist> {
    pub fn music_song(mut self, music_song: url::Url) -> Self {
        self.list
            .push(("music:song", OpenGraphDataTypes::Url(music_song)));
        self
    }

    pub fn music_song_disc(mut self, music_song_disc: isize) -> Self {
        if music_song_disc >= 1 {
            self.list.push((
                "music:song:disc",
                OpenGraphDataTypes::Integer(music_song_disc),
            ));
        }
        self
    }

    pub fn music_song_track(mut self, music_song_track: isize) -> Self {
        if music_song_track >= 1 {
            self.list.push((
                "music:song:track",
                OpenGraphDataTypes::Integer(music_song_track),
            ));
        }
        self
    }

    pub fn music_creator(mut self, music_creator: url::Url) -> Self {
        self.list
            .push(("music:creator", OpenGraphDataTypes::Url(music_creator)));
        self
    }
}

impl OpenGraphBuilder<MusicRadioStation> {
    pub fn music_creator(mut self, music_creator: url::Url) -> Self {
        self.list
            .push(("music:creator", OpenGraphDataTypes::Url(music_creator)));
        self
    }
}

impl OpenGraphBuilder<VideoMovie> {
    pub fn video_actor(mut self, video_actor: url::Url) -> Self {
        self.list
            .push(("video:actor", OpenGraphDataTypes::Url(video_actor)));
        self
    }

    pub fn video_actor_role(mut self, video_actor_role: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:actor:role",
            OpenGraphDataTypes::String(video_actor_role.into().to_string()),
        ));
        self
    }

    pub fn video_director(mut self, video_director: url::Url) -> Self {
        self.list
            .push(("video:director", OpenGraphDataTypes::Url(video_director)));
        self
    }

    pub fn video_writer(mut self, video_writer: url::Url) -> Self {
        self.list
            .push(("video:writer", OpenGraphDataTypes::Url(video_writer)));
        self
    }

    pub fn video_duration(mut self, video_duration: isize) -> Self {
        if video_duration >= 1 {
            self.list.push((
                "video:duration",
                OpenGraphDataTypes::Integer(video_duration),
            ));
        }
        self
    }

    pub fn video_release_date(mut self, video_release_date: SystemTime) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(format_rfc3339(video_release_date).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn video_release_date_chrono(
        mut self,
        video_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(video_release_date.into().to_rfc3339()),
        ));
        self
    }

    pub fn video_tag(mut self, video_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:tag",
            OpenGraphDataTypes::String(video_tag.into().to_string()),
        ));
        self
    }
}

impl OpenGraphBuilder<VideoEpisode> {
    pub fn video_actor(mut self, video_actor: url::Url) -> Self {
        self.list
            .push(("video:actor", OpenGraphDataTypes::Url(video_actor)));
        self
    }

    pub fn video_actor_role(mut self, video_actor_role: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:actor:role",
            OpenGraphDataTypes::String(video_actor_role.into().to_string()),
        ));
        self
    }

    pub fn video_director(mut self, video_director: url::Url) -> Self {
        self.list
            .push(("video:director", OpenGraphDataTypes::Url(video_director)));
        self
    }

    pub fn video_writer(mut self, video_writer: url::Url) -> Self {
        self.list
            .push(("video:writer", OpenGraphDataTypes::Url(video_writer)));
        self
    }

    pub fn video_duration(mut self, video_duration: isize) -> Self {
        if video_duration >= 1 {
            self.list.push((
                "video:duration",
                OpenGraphDataTypes::Integer(video_duration),
            ));
        }
        self
    }

    pub fn video_release_date(mut self, video_release_date: SystemTime) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(format_rfc3339(video_release_date).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn video_release_date_chrono(
        mut self,
        video_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(video_release_date.into().to_rfc3339()),
        ));
        self
    }

    pub fn video_tag(mut self, video_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:tag",
            OpenGraphDataTypes::String(video_tag.into().to_string()),
        ));
        self
    }

    pub fn video_series(mut self, video_series: url::Url) -> Self {
        self.list
            .push(("video:series", OpenGraphDataTypes::Url(video_series)));
        self
    }
}

impl OpenGraphBuilder<VideoTvShow> {
    pub fn video_actor(mut self, video_actor: url::Url) -> Self {
        self.list
            .push(("video:actor", OpenGraphDataTypes::Url(video_actor)));
        self
    }

    pub fn video_actor_role(mut self, video_actor_role: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:actor:role",
            OpenGraphDataTypes::String(video_actor_role.into().to_string()),
        ));
        self
    }

    pub fn video_director(mut self, video_director: url::Url) -> Self {
        self.list
            .push(("video:director", OpenGraphDataTypes::Url(video_director)));
        self
    }

    pub fn video_writer(mut self, video_writer: url::Url) -> Self {
        self.list
            .push(("video:writer", OpenGraphDataTypes::Url(video_writer)));
        self
    }

    pub fn video_duration(mut self, video_duration: isize) -> Self {
        if video_duration >= 1 {
            self.list.push((
                "video:duration",
                OpenGraphDataTypes::Integer(video_duration),
            ));
        }
        self
    }

    #[cfg(feature = "chrono")]
    pub fn video_release_date_chrono(
        mut self,
        video_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(video_release_date.into().to_rfc3339()),
        ));
        self
    }

    pub fn video_tag(mut self, video_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:tag",
            OpenGraphDataTypes::String(video_tag.into().to_string()),
        ));
        self
    }
}

impl OpenGraphBuilder<VideoOther> {
    pub fn video_actor(mut self, video_actor: url::Url) -> Self {
        self.list
            .push(("video:actor", OpenGraphDataTypes::Url(video_actor)));
        self
    }

    pub fn video_actor_role(mut self, video_actor_role: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:actor:role",
            OpenGraphDataTypes::String(video_actor_role.into().to_string()),
        ));
        self
    }

    pub fn video_director(mut self, video_director: url::Url) -> Self {
        self.list
            .push(("video:director", OpenGraphDataTypes::Url(video_director)));
        self
    }

    pub fn video_writer(mut self, video_writer: url::Url) -> Self {
        self.list
            .push(("video:writer", OpenGraphDataTypes::Url(video_writer)));
        self
    }

    pub fn video_duration(mut self, video_duration: isize) -> Self {
        if video_duration >= 1 {
            self.list.push((
                "video:duration",
                OpenGraphDataTypes::Integer(video_duration),
            ));
        }
        self
    }

    pub fn video_release_date(mut self, video_release_date: SystemTime) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(format_rfc3339(video_release_date).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn video_release_date_chrono(
        mut self,
        video_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "video:release_date",
            OpenGraphDataTypes::String(video_release_date.into().to_rfc3339()),
        ));
        self
    }

    pub fn video_tag(mut self, video_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "video:tag",
            OpenGraphDataTypes::String(video_tag.into().to_string()),
        ));
        self
    }
}

impl OpenGraphBuilder<Article> {
    pub fn article_published_time(mut self, article_published_time: SystemTime) -> Self {
        self.list.push((
            "article:published_time",
            OpenGraphDataTypes::String(format_rfc3339(article_published_time).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn article_published_time_chrono(
        mut self,
        article_published_time: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "article:published_time",
            OpenGraphDataTypes::String(article_published_time.into().to_rfc3339()),
        ));
        self
    }

    pub fn article_modified_time(mut self, article_modified_time: SystemTime) -> Self {
        self.list.push((
            "article:modified_time",
            OpenGraphDataTypes::String(format_rfc3339(article_modified_time).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn article_modified_time_chrono(
        mut self,
        article_modified_time: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "article:modified_time",
            OpenGraphDataTypes::String(article_modified_time.into().to_rfc3339()),
        ));
        self
    }

    pub fn article_expiration_time(mut self, article_expiration_time: SystemTime) -> Self {
        self.list.push((
            "article:expiration_time",
            OpenGraphDataTypes::String(format_rfc3339(article_expiration_time).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn article_expiration_time_chrono(
        mut self,
        article_expiration_time: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "article:expiration_time",
            OpenGraphDataTypes::String(article_expiration_time.into().to_rfc3339()),
        ));
        self
    }

    pub fn article_author(mut self, article_author: url::Url) -> Self {
        self.list
            .push(("article:author", OpenGraphDataTypes::Url(article_author)));
        self
    }

    pub fn article_section(mut self, article_section: impl Into<&'static str>) -> Self {
        self.list.push((
            "article:section",
            OpenGraphDataTypes::String(article_section.into().to_string()),
        ));
        self
    }

    pub fn article_tag(mut self, article_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "article:tag",
            OpenGraphDataTypes::String(article_tag.into().to_string()),
        ));
        self
    }
}

impl OpenGraphBuilder<Book> {
    pub fn book_author(mut self, book_author: url::Url) -> Self {
        self.list
            .push(("book:author", OpenGraphDataTypes::Url(book_author)));
        self
    }

    pub fn book_isbn(mut self, book_isbn: impl Into<&'static str>) -> Self {
        self.list.push((
            "book:isbn",
            OpenGraphDataTypes::String(book_isbn.into().to_string()),
        ));
        self
    }

    pub fn book_release_date(mut self, book_release_date: SystemTime) -> Self {
        self.list.push((
            "book:release_date",
            OpenGraphDataTypes::String(format_rfc3339(book_release_date).to_string()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn book_release_data_chrono(
        mut self,
        book_release_date: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "book:release_date",
            OpenGraphDataTypes::String(book_release_date.into().to_rfc3339()),
        ));
        self
    }

    pub fn book_tag(mut self, book_tag: impl Into<&'static str>) -> Self {
        self.list.push((
            "book:tag",
            OpenGraphDataTypes::String(book_tag.into().to_string()),
        ));
        self
    }
}

impl OpenGraphBuilder<PaymentLink> {
    pub fn payment_description(mut self, payment_description: impl Into<&'static str>) -> Self {
        self.list.push((
            "payment:description",
            OpenGraphDataTypes::String(payment_description.into().to_string()),
        ));
        self
    }

    pub fn payment_currency(mut self, payment_currency: impl Into<&'static str>) -> Self {
        self.list.push((
            "payment:currency",
            OpenGraphDataTypes::String(payment_currency.into().to_string()),
        ));
        self
    }

    pub fn payment_amount(mut self, payment_amount: impl Into<f64>) -> Self {
        self.list.push((
            "payment:amount",
            OpenGraphDataTypes::Float(payment_amount.into()),
        ));
        self
    }

    #[cfg(feature = "chrono")]
    pub fn payment_expires_at_chrono(
        mut self,
        payment_expires_at: impl Into<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.list.push((
            "payment:expires_at",
            OpenGraphDataTypes::String(payment_expires_at.into().to_rfc3339()),
        ));
        self
    }

    pub fn payment_status(mut self, payment_status: PaymentStatus) -> Self {
        self.list.push((
            "payment:status",
            OpenGraphDataTypes::String(payment_status.to_string()),
        ));
        self
    }

    pub fn payment_id(mut self, payment_id: impl Into<&'static str>) -> Self {
        self.list.push((
            "payment:id",
            OpenGraphDataTypes::String(payment_id.into().to_string()),
        ));
        self
    }

    pub fn payment_success_url(mut self, payment_success_url: url::Url) -> Self {
        self.list.push((
            "payment:success_url",
            OpenGraphDataTypes::Url(payment_success_url),
        ));
        self
    }
}

impl OpenGraphBuilder<Profile> {
    pub fn profile_first_name(mut self, profile_first_name: impl Into<&'static str>) -> Self {
        self.list.push((
            "profile:first_name",
            OpenGraphDataTypes::String(profile_first_name.into().to_string()),
        ));
        self
    }

    pub fn profile_last_name(mut self, profile_last_name: impl Into<&'static str>) -> Self {
        self.list.push((
            "profile:last_name",
            OpenGraphDataTypes::String(profile_last_name.into().to_string()),
        ));
        self
    }

    pub fn profile_username(mut self, profile_username: impl Into<&'static str>) -> Self {
        self.list.push((
            "profile:username",
            OpenGraphDataTypes::String(profile_username.into().to_string()),
        ));
        self
    }

    pub fn profile_gender(mut self, profile_gender: Gender) -> Self {
        self.list.push((
            "profile:gender",
            OpenGraphDataTypes::String(profile_gender.to_string()),
        ));
        self
    }
}

#[derive(Debug, Clone)]
pub struct OpenGraph {
    pub prefix: String,
    pub og_type: String,
    pub list: Box<[(String, String)]>,
}
