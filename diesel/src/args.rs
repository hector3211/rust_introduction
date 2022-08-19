use clap::{Args, Parser, SubCommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct TestArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, SubCommand)]
pub enum EntityType {
    User(UserCommand),
    Video(VideoCommand),
    View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    // name of user
    pub name: String,
    // email of user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    // id of user
    pub id: i32,
    // name of user
    pub name: String,
    // email of user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    // id of entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub subcommand: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    Create(CreateVideo),
    Update(UpdateVideo),
    Delete(DeleteVideo),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    // title of video
    pub title: String,
    // description of video
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    // id of video
    pub id: i32,
    // title of video
    pub title: String,
    // description of video
    pub description: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub subcommand: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    Create(CreateView),
    Show,
    ShowPretty,
}

#[derive(Debug, Args)]
pub struct CreateView {
    // id of video
    pub video_id: i32,
    // id of user
    pub user_id: i32,
    // time of view
    pub watch_start: chrono::NaiveDateTime,
    // how long the user watched the video
    pub watch_duration: i32,
}
