use crate::args::{CreateVideo, DeleteEntity, UpdateVideo, VideoCommand, VideoSubcommand};
use crate::db::establish_connection;
use crate::models::{NewVideo, Video};
use diesel::prelude::*;

pub fn create_video(video: CreateVideo) {
    println!("Creating video: {:?}", video);
    use crate::schema::videos::dsl::*;
    let connection = establish_connection();
    let new_video = NewVideo {
        title: &video.title,
        description: &video.description,
        removed: false,
    };
    diesel::insert_into(videos)
        .values(&new_video)
        .execute(&connection)
        .expect("Error saving new video");
}

pub fn update_video(video: UpdateVideo) {
    use crate::schema::videos::dsl::*;
    let connection = establish_connection();
    let updated_video = Video {
        id: video.id,
        title: video.title,
        description: video.description,
        removed: false,
    };
    diesel::update(videos.find(video.id))
        .set(&db_video)
        .execute(&connection)
        .expect("Error updating video");
}

pub fn show_videos() {
    println!("Showing videos");
    use crate::schema::videos::dsl::*;
    let connection = establish_connection();
    let results = videos
        .filter(removed.eq(false))
        .load::<Video>(&connection)
        .expect("Error loading videos");
    println!("Displaying {} videos", results.len());
    for video in results {
        println!("{}", video);
    }
}
