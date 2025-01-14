pub mod music {
	pub mod get_cover_image;
	pub mod get_music;
	pub mod increment_times_played;
	pub mod save_music;
	pub mod search_music;
	pub mod send_music;
}
pub mod playlist {
	pub mod add_song_to_playlist;
	pub mod create_new_playlist;
	pub mod get_playlist_music;
	pub mod get_users_playlists;
}
pub mod users {
	pub mod add_friend;
	pub mod get_user;
	pub mod get_user_pfp;
	pub mod remove_friend;
	pub mod update_pfp;
	pub mod search_user;
}
pub mod auth {
	pub mod login;
	pub mod logout;
	pub mod signup;
	pub mod verify;
}
pub mod socket;
