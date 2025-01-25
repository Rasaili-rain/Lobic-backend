use axum::http::{request::Parts, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const IP: &str = "127.0.0.1";
pub const PORT: &str = "8080";
pub const COVER_IMG_STORAGE: &str = "./storage/cover_images";
pub const MUSIC_STORAGE: &str = "./storage/music_db";
pub const USER_PFP_STORAGE: &str = "./storage/users_pfps";
pub const PLAYLIST_COVER_IMG_STORAGE: &str = "./storage/playlists_cover_img";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpCode {
	OK,
	ERROR,
	CONNECT,
	#[allow(non_camel_case_types)]
	CREATE_LOBBY,
	#[allow(non_camel_case_types)]
	JOIN_LOBBY,
	#[allow(non_camel_case_types)]
	LEAVE_LOBBY,
	#[allow(non_camel_case_types)]
	DELETE_LOBBY,
	#[allow(non_camel_case_types)]
	GET_LOBBY_IDS,
	#[allow(non_camel_case_types)]
	GET_LOBBY_MEMBERS,
	MESSAGE,
	#[allow(non_camel_case_types)]
	GET_MESSAGES,
	#[allow(non_camel_case_types)]
	SET_MUSIC_STATE,
	#[allow(non_camel_case_types)]
	SYNC_MUSIC,
	#[allow(non_camel_case_types)]
	SET_QUEUE,
	#[allow(non_camel_case_types)]
	SYNC_QUEUE,
	#[allow(non_camel_case_types)]
	ADD_FRIEND,
	#[allow(non_camel_case_types)]
	REMOVE_FRIEND,
	NOTIFICATION,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum MusicState {
	PLAY,
	PAUSE,
	#[allow(non_camel_case_types)]
	CHANGE_MUSIC,
	#[allow(non_camel_case_types)]
	CHANGE_TIME,
	#[allow(non_camel_case_types)]
	CHANGE_VOLUME,
	EMPTY,
}

pub fn allowed_origins(origin: &HeaderValue, _request: &Parts) -> bool {
	let origins = [
		"http://localhost:5173",
		"http://127.0.0.1:5173",
		"http://localhost:5174",
		"http://127.0.0.1:5174",
		"http://localhost:5175",
		"http://127.0.0.1:5175",
	];
	origins.iter().any(|&allowed| origin == allowed)
}

// Structure for WebSocket

// Request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocketPayload {
	pub op_code: OpCode,
	pub value: Value,
}

// Response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocketResponse {
	pub op_code: OpCode,
	pub r#for: OpCode,
	pub value: Value,
}

impl SocketResponse {
	pub fn to_string(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}
