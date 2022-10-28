///
/// A trait that
///
pub trait Repository<A: App>
where
    Self::StringRet: Into<String>,
{
    type StringRet;
    fn get_apps(&self) -> Vec<A>;
    fn name(&self) -> Option<&Self::StringRet>;
}

pub trait Backend<A: App, R: Repository<A>> {
    fn get_repositories() -> Vec<R>;
    fn get_apps(repos: &Vec<R>) -> Vec<A>;
}

use std::{
    io,
    process::{ExitCode, ExitStatus},
};

/// A trait that represents all kinds of Flatpak Application!
///
use async_trait::async_trait;
#[async_trait]
pub trait App
where
    Self::StringRet: Into<String>,
{
    type StringRet;

    fn id(&self) -> &Self::StringRet;
    fn author(&self) -> Option<&Self::StringRet>;

    fn description(&self) -> Option<&Self::StringRet>;

    fn images(&self) -> Vec<&Self::StringRet>;

    fn title(&self) -> &Self::StringRet;

    async fn install(&self) -> io::Result<ExitStatus>;
    async fn install_id(id: &str) -> io::Result<ExitStatus>;
}
