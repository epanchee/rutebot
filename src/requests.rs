use std::error::Error as StdError;
use std::ops::Not;

use hyper::Body;
use hyper_multipart_rfc7578::client::multipart;
use hyper_multipart_rfc7578::client::multipart::Form;
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::error::Error;

/// Contains types for sending [getUpdates](https://core.telegram.org/bots/api#getupdates) request
pub mod get_updates;

/// Contains types for sending [getFile](https://core.telegram.org/bots/api#getfile) request
pub mod get_file;

/// Contains types for sending [getMe](https://core.telegram.org/bots/api#getMe) request
pub mod get_me;

/// Contains types for sending [sendMessage](https://core.telegram.org/bots/api#sendMessage) request
pub mod send_text;

/// Contains types for sending [sendDocument](https://core.telegram.org/bots/api#senddocument) request
pub mod send_document;

/// Contains types for sending [sendPhoto](https://core.telegram.org/bots/api#sendphoto) request
pub mod send_photo;

/// Contains types for sending [sendAudio](https://core.telegram.org/bots/api#sendaudio) request
pub mod send_audio;

/// Contains types for sending [sendVideo](https://core.telegram.org/bots/api#sendvideo) request
pub mod send_video;

/// Contains types for sending [sendAnimation](https://core.telegram.org/bots/api#sendanimation) request
pub mod send_animation;

/// Contains types for sending [sendVoice](https://core.telegram.org/bots/api#sendvoice) request
pub mod send_voice;

/// Contains types for sending [sendVideoNote](https://core.telegram.org/bots/api#sendvideonote) request
pub mod send_video_note;

/// Contains types for sending [sendMediaGroup](https://core.telegram.org/bots/api#sendmediagroup) request
pub mod send_media_group;

/// Contains types for sending [sendLocation](https://core.telegram.org/bots/api#sendlocation) request
pub mod send_location;

/// Contains types for sending [editMessageLiveLocation](https://core.telegram.org/bots/api#editmessagelivelocation) request
pub mod edit_live_location;

/// Contains types for sending [stopMessageLiveLocation](https://core.telegram.org/bots/api#stopmessagelivelocation) request
pub mod stop_live_location;

/// Contains types for sending [sendVenue](https://core.telegram.org/bots/api#sendvenue) request
pub mod send_venue;

/// Contains types for sending [sendContact](https://core.telegram.org/bots/api#sendcontact) request
pub mod send_contact;

/// Contains types for sending [sendPoll](https://core.telegram.org/bots/api#sendpoll) request
pub mod send_poll;

/// Contains types for sending [sendChatAction](https://core.telegram.org/bots/api#sendchataction) request
pub mod send_chat_action;

/// Contains types for sending [answerCallbackQuery](https://core.telegram.org/bots/api#answercallbackquery) request
pub mod answer_callback_query;

/// Contains types for sending [forwardMessage](https://core.telegram.org/bots/api#forwardmessage) request
pub mod forward_message;

/// Contains types for sending [getUserProfilePhotos](https://core.telegram.org/bots/api#getuserprofilephotos)
pub mod get_user_profile_photos;

/// Contains types for sending [restrictChatMember](https://core.telegram.org/bots/api#restrictchatmember) request
pub mod restrict_chat_member;

/// Contains types for sending [unbanChatMember](https://core.telegram.org/bots/api#unbanchatmember) request
pub mod unban_chat_member;

/// Contains types for sending [kickChatMember](https://core.telegram.org/bots/api#kickchatmember) request
pub mod kick_chat_member;

/// Contains types for sending [promoteChatMember](https://core.telegram.org/bots/api#promotechatmember) request
pub mod promote_chat_member;

/// Contains types for sending [exportChatInviteLink](https://core.telegram.org/bots/api#exportchatinvitelink) request
pub mod export_chat_invite_link;

/// Contains types for sending [setChatPhoto](https://core.telegram.org/bots/api#setchatphoto) request
pub mod set_chat_photo;

/// Contains types for sending [deleteChatPhoto](https://core.telegram.org/bots/api#deletechatphoto) request
pub mod delete_chat_photo;

/// Contains types for sending [setChatTitle](https://core.telegram.org/bots/api#setchattitle) request
pub mod set_chat_title;

/// Contains types for sending [setChatDescription](https://core.telegram.org/bots/api#setchatdescription) request
pub mod set_chat_description;

/// Contains types for sending [pinChatMessage](https://core.telegram.org/bots/api#pinchatmessage) request
pub mod pin_chat_message;

/// Contains types for sending [unpinChatMessage](https://core.telegram.org/bots/api#unpinchatmessage) request
pub mod unpin_chat_message;

/// Contains types for sending [leaveChat](https://core.telegram.org/bots/api#leavechat) request
pub mod leave_chat;

/// Contains types for sending [getChat](https://core.telegram.org/bots/api#getchat) request
pub mod get_chat;

/// Contains types for sending [getChatAdministrators](https://core.telegram.org/bots/api#getchatadministrators) request
pub mod get_chat_administrators;

/// Contains types for sending [getChatMembersCount](https://core.telegram.org/bots/api#getchatmemberscount) request
pub mod get_chat_members_count;

/// Contains types for sending [getChatMember](https://core.telegram.org/bots/api#getchatmember) request
pub mod get_chat_member;

/// Contains types for sending for [setChatStickerSet](https://core.telegram.org/bots/api#setchatstickerset) request
pub mod set_chat_sticker_set;

/// Contains types for sending [deleteChatStickerSet](https://core.telegram.org/bots/api#deletechatstickerset) request
pub mod delete_chat_sticker_set;

/// Contains types for sending [editMessageText](https://core.telegram.org/bots/api#editmessagetext) request
pub mod edit_message_text;

/// Contains types for sending [editMessageCaption](https://core.telegram.org/bots/api#editmessagecaption) request
pub mod edit_message_caption;

/// Contains types for sending [editMessageMedia](https://core.telegram.org/bots/api#editmessagemedia) request
pub mod edit_message_media;

/// Contains types for sending [editMessageReplyMarkup](https://core.telegram.org/bots/api#editmessagereplymarkup) request
pub mod edit_message_reply_markup;

/// Contains types for sending [stopPoll](https://core.telegram.org/bots/api#stoppoll) request
pub mod stop_poll;

/// Contains types for sending [deleteMessage](https://core.telegram.org/bots/api#deletemessage) request
pub mod delete_message;

/// Basic request type.
pub trait Request: Serialize + Sized {
    type ResponseType;

    fn method(&self) -> &'static str;

    fn set_http_request_body(
        self,
        request_builder: hyper::http::request::Builder,
    ) -> Result<hyper::http::request::Request<Body>, Error> {
        add_json_body(request_builder, &self)
    }
}

pub(crate) fn add_json_body<S: Serialize + Sized>(
    mut request_builder: hyper::http::request::Builder,
    serializable: &S,
) -> Result<hyper::http::request::Request<Body>, Error> {
    let json_bytes = serde_json::to_vec(serializable).map_err(Error::Serde)?;
    request_builder
        .header("content-type", "application/json")
        .body(Body::from(json_bytes))
        .map_err(|x| Error::RequestBuild(x.description().to_string()))
}

pub(crate) fn add_form_body(
    mut request_builder: hyper::http::request::Builder,
    form: Form<'static>,
) -> Result<hyper::http::request::Request<Body>, Error> {
    form.set_body_convert::<hyper::Body, multipart::Body>(&mut request_builder)
        .map_err(|x| Error::RequestBuild(x.description().to_string()))
}

pub(crate) fn add_fields_to_form<S: Serialize + Sized>(
    form: &mut Form<'static>,
    serializable: &S,
) -> Result<(), Error> {
    let json = serde_json::to_value(serializable).map_err(Error::Serde)?;
    if let Value::Object(map) = json {
        for (k, v) in map {
            match v {
                Value::String(s) => form.add_text(k, s),
                other => form.add_text(k, other.to_string()),
            }
        }
    }
    Ok(())
}

/// File to send
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FileKind<'a> {
    /// Identifier of file on the telegram servers
    FileId(&'a str),

    /// Http url for the file to be sent. Telegram will download and send the file.
    /// 5 MB max size for photos and 20 MB max for other types of content
    Url(&'a str),

    /// Arbitrary file to be uploaded
    #[serde(serialize_with = "FileKind::serialize_attach")]
    InputFile {
        /// Name of the file
        name: &'a str,
        /// File content
        content: Vec<u8>,
    },
}

impl<'a> FileKind<'a> {
    pub(crate) fn is_input_file(&self) -> bool {
        if let FileKind::InputFile { .. } = &self {
            true
        } else {
            false
        }
    }

    pub(crate) fn serialize_attach<S: Serializer>(
        field0: &str,
        _: &[u8],
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("attach://{}", field0))
    }
}

/// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId<'a> {
    /// Unique identifier for the target group
    Id(i64),
    /// Username of the target supergroup or channel (in the format @channelusername)
    Username(&'a str),
}

impl<'a> From<i64> for ChatId<'a> {
    fn from(x: i64) -> Self {
        ChatId::Id(x)
    }
}

impl<'a> From<&'a str> for ChatId<'a> {
    fn from(x: &'a str) -> Self {
        ChatId::Username(x)
    }
}

/// Represents a photo to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaPhoto<'a, 'b> {
    /// File to send
    pub media: FileKind<'a>,

    /// Caption of the photo to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'b str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

impl<'a, 'b> InputMediaPhoto<'a, 'b> {
    pub fn new(media: FileKind<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
        }
    }
}

/// Represents a video to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaVideo<'a, 'b> {
    /// File to send
    pub media: FileKind<'a>,

    /// Caption of the photo to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'b str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Pass True, if the uploaded video is suitable for streaming.
    #[serde(skip_serializing_if = "Not::not")]
    pub supports_streaming: bool,
}

impl<'a, 'b> InputMediaVideo<'a, 'b> {
    pub fn new(media: FileKind<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            duration: None,
            width: None,
            height: None,
            supports_streaming: false,
        }
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAnimation<'a, 'b> {
    /// File to send
    pub media: FileKind<'a>,

    /// Caption of the animation to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'b str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Animation duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

impl<'a, 'b> InputMediaAnimation<'a, 'b> {
    pub fn new(media: FileKind<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            duration: None,
            width: None,
            height: None,
        }
    }
}

/// Represents a general file to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaDocument<'a, 'b> {
    /// File to send
    pub media: FileKind<'a>,

    /// Caption of the document to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'b str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

impl<'a, 'b> InputMediaDocument<'a, 'b> {
    pub fn new(media: FileKind<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
        }
    }
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAudio<'a, 'b, 'c, 'd> {
    /// File to send
    pub media: FileKind<'a>,

    /// Caption of the audio to be sent, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'b str>,

    /// Send `ParseMode::Markdown` or `ParseMode::Html`,
    /// if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options) in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<&'c str>,

    /// Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'d str>,
}

impl<'a, 'b, 'c, 'd> InputMediaAudio<'a, 'b, 'c, 'd> {
    pub fn new(media: FileKind<'a>) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputMedia<'a, 'b, 'c, 'd> {
    #[serde(rename = "video")]
    Video(InputMediaVideo<'a, 'b>),

    #[serde(rename = "photo")]
    Photo(InputMediaPhoto<'a, 'b>),

    #[serde(rename = "animation")]
    Animation(InputMediaAnimation<'a, 'b>),

    #[serde(rename = "document")]
    Document(InputMediaDocument<'a, 'b>),

    #[serde(rename = "audio")]
    Audio(InputMediaAudio<'a, 'b, 'c, 'd>),
}

impl<'a, 'b, 'c, 'd> InputMedia<'a, 'b, 'c, 'd> {
    fn get_file(self) -> FileKind<'a> {
        match self {
            InputMedia::Photo(x) => x.media,
            InputMedia::Video(x) => x.media,
            InputMedia::Animation(x) => x.media,
            InputMedia::Document(x) => x.media,
            InputMedia::Audio(x) => x.media,
        }
    }

    fn contains_input_file(&self) -> bool {
        match &self {
            InputMedia::Video(x) => x.media.is_input_file(),
            InputMedia::Photo(x) => x.media.is_input_file(),
            InputMedia::Animation(x) => x.media.is_input_file(),
            InputMedia::Document(x) => x.media.is_input_file(),
            InputMedia::Audio(x) => x.media.is_input_file(),
        }
    }
}

/// Additional interface options
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup<'a, 'b, 'c> {
    InlineKeyboard(InlineKeyboard<'a, 'b, 'c>),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup<'a, 'b>),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// This object represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating)
/// that appears right next to the message it belongs to
#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboard<'a, 'b, 'c> {
    /// Array of button rows, each represented by an Array of `InlineKeyboardButton` objects
    pub inline_keyboard: &'c [Vec<InlineKeyboardButton<'a, 'b>>],
}

/// This object represents a custom keyboard with reply options.
#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup<'a, 'b> {
    /// Array of button rows, each represented by an Array of `KeyboardButton` objects
    pub keyboard: &'b [Vec<KeyboardButton<'a>>],

    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,

    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically display the usual
    /// letter-keyboard in the chat – the user can press a special button in the input field to
    /// see the custom keyboard again. Defaults to false
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    /// Use this parameter if you want to show the keyboard to specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2)
    /// if the bot's message is a reply (has `reply_to_message_id`), sender of the original message.

    /// Example: A user requests to change the bot‘s language, bot replies to the request with a
    /// keyboard to select the new language. Other users in the group don’t see the keyboard
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard
/// and display the default letter-keyboard. By default, custom keyboards are displayed until a
/// new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden
/// immediately after the user presses a button (see `ReplyKeyboardMarkup`)
#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard
    /// (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible,
    /// use `one_time_keyboard` in `ReplyKeyboardMarkup`)
    #[serde(skip_serializing_if = "Not::not")]
    pub remove_keyboard: bool,

    /// Use this parameter if you want to remove the keyboard for specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's
    /// message is a reply (has `reply_to_message_id`), sender of the original message.
    ///
    /// Example: A user votes in a poll, bot returns confirmation message in reply
    /// to the vote and removes the keyboard for that user, while still showing the keyboard
    /// with poll options to users who haven't voted yet
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface
/// to the user (act as if the user has selected the bot‘s message and tapped ’Reply').
/// This can be extremely useful if you want to create user-friendly step-by-step interfaces
/// without having to sacrifice [privacy mode](https://core.telegram.org/bots#privacy-mode).
#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot‘s message and tapped ’Reply'
    #[serde(skip_serializing_if = "Not::not")]
    pub force_reply: bool,
    /// Use this parameter if you want to force reply from specific users only.
    /// Targets: 1) users that are @mentioned in the text of the Message object; 2)
    /// if the bot's message is a reply (has `reply_to_message_id`), sender of the original message
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InlineKeyboardButton<'a, 'b> {
    Url {
        /// Label text on the button
        text: &'a str,
        /// HTTP or tg:// url to be opened when button is pressed
        url: &'b str,
    },
    CallbackData {
        /// Label text on the button
        text: &'a str,
        /// Data to be sent in a [callback query](https://core.telegram.org/bots/api#callbackquery)
        /// to the bot when button is pressed, 1-64 bytes
        callback_data: &'b str,
    },
}

/// This object represents one button of the reply keyboard
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeyboardButton<'a> {
    /// Text of the button. It will be sent as a message when the button is pressed
    Text(&'a str),
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown,
}

#[serde(untagged)]
#[derive(Serialize, Debug, Clone)]
pub enum MessageOrInlineMessageId<'a> {
    Inline {
        inline_message_id: &'a str,
    },
    Chat {
        chat_id: ChatId<'a>,
        message_id: i64,
    },
}
