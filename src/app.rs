use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks}, bgworker::Queue, boot::{create_app, BootResult, StartMode}, cache, controller::AppRoutes, environment::Environment, task::Tasks, Result
};
use migration::Migrator;
use sea_orm::DatabaseConnection;
use std::path::Path;

use crate::controllers;
#[allow(unused_imports)]
use crate::tasks;

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::empty() // controller routes below
            .add_route(controllers::home::routes())
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    async fn truncate(_db: &DatabaseConnection) -> Result<()> {
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _base: &Path) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks.register(TASK);
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(AppContext{
            cache: cache::Cache::new(cache::drivers::inmem::new()).into(),
            ..ctx
        })
    }
}
