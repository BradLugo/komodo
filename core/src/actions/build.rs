use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use diff::Diff;
use helpers::to_monitor_name;
use mungos::{doc, to_bson};
use types::{
    traits::Permissioned, Build, Log, Operation, PermissionLevel, Update, UpdateStatus,
    UpdateTarget,
};

use crate::{
    auth::RequestUser,
    helpers::{all_logs_success, any_option_diff_is_some, option_diff_is_some},
    state::State,
};

impl State {
    pub async fn get_build_check_permissions(
        &self,
        build_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Build> {
        let build = self.db.get_build(build_id).await?;
        let permissions = build.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(build)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this build"
            ))
        }
    }

    pub async fn create_build(
        &self,
        name: &str,
        server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        self.get_server_check_permissions(&server_id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;
        let build = Build {
            name: to_monitor_name(name),
            server_id,
            permissions: [(user.id.clone(), PermissionLevel::Write)]
                .into_iter()
                .collect(),
            created_at: start_ts,
            updated_at: start_ts,
            ..Default::default()
        };
        let start_ts = unix_timestamp_ms() as i64;
        let build_id = self
            .db
            .builds
            .create_one(build)
            .await
            .context("failed at adding build to db")?;
        let build = self.db.get_build(&build_id).await?;
        let update = Update {
            target: UpdateTarget::Build(build_id),
            operation: Operation::CreateBuild,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(build)
    }

    pub async fn delete_build(&self, build_id: &str, user: &RequestUser) -> anyhow::Result<Build> {
        let build = self
            .get_build_check_permissions(build_id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;
        let server = self.db.get_server(&build.server_id).await?;
        let delete_repo_log = self
            .periphery
            .delete_repo(&server, &build.name)
            .await
            .context("failed at deleting repo")?;
        self.db.builds.delete_one(build_id).await?;
        let update = Update {
            target: UpdateTarget::System,
            operation: Operation::DeleteDeployment,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
            operator: user.id.clone(),
            logs: vec![
                delete_repo_log,
                Log::simple(format!(
                    "deleted build {} on server {}",
                    build.name, server.name
                )),
            ],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(build)
    }

    pub async fn update_build(
        &self,
        mut new_build: Build,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        let current_build = self
            .get_build_check_permissions(&new_build.id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;
        new_build.permissions = current_build.permissions.clone();
        new_build.updated_at = start_ts;

        self.db
            .builds
            .update_one(&new_build.id, mungos::Update::Regular(new_build.clone()))
            .await
            .context("failed at update one build")?;

        let diff = current_build.diff(&new_build);

        let mut update = Update {
            operation: Operation::UpdateBuild,
            target: UpdateTarget::Build(new_build.id.clone()),
            start_ts,
            status: UpdateStatus::InProgress,
            logs: vec![Log::simple(serde_json::to_string_pretty(&diff).unwrap())],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        if any_option_diff_is_some(&[&diff.repo, &diff.branch, &diff.github_account])
            || option_diff_is_some(&diff.on_clone)
        {
            let server = self.db.get_server(&current_build.server_id).await?;
            match self.periphery.clone_repo(&server, &new_build).await {
                Ok(clone_logs) => {
                    update.logs.extend(clone_logs);
                }
                Err(e) => update
                    .logs
                    .push(Log::error("cloning repo", format!("{e:#?}"))),
            }
        }

        update.end_ts = Some(unix_timestamp_ms() as i64);
        update.success = all_logs_success(&update.logs);
        update.status = UpdateStatus::Complete;

        self.update_update(update).await?;
        Ok(new_build)
    }

    pub async fn build(&self, build_id: &str, user: &RequestUser) -> anyhow::Result<Update> {
        let mut build = self
            .get_build_check_permissions(build_id, user, PermissionLevel::Write)
            .await?;
        let server = self.db.get_server(&build.server_id).await?;

        build.version.increment();

        let mut update = Update {
            target: UpdateTarget::Build(build_id.to_string()),
            operation: Operation::BuildBuild,
            start_ts: unix_timestamp_ms() as i64,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            version: build.version.clone().into(),
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let build_logs = self
            .periphery
            .build(&server, &build)
            .await
            .context("failed at call to periphery to build")?;

        match build_logs {
            Some(logs) => {
                update.logs.extend(logs);
                update.success = all_logs_success(&update.logs);
                if update.success {
                    self.db
                        .builds
                        .update_one::<Build>(
                            build_id,
                            mungos::Update::Set(doc! {
                                "version": to_bson(&build.version)
                                    .context("failed at converting version to bson")?
                            }),
                        )
                        .await?;
                }
            }
            None => {
                update
                    .logs
                    .push(Log::error("build", "builder busy".to_string()));
            }
        }
        update.status = UpdateStatus::Complete;
        update.end_ts = Some(unix_timestamp_ms() as i64);
        self.update_update(update.clone()).await?;
        Ok(update)
    }

    pub async fn reclone_build(
        &self,
        build_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let build = self
            .get_build_check_permissions(build_id, user, PermissionLevel::Write)
            .await?;
        let server = self.db.get_server(&build.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Build(build_id.to_string()),
            operation: Operation::RecloneBuild,
            start_ts: unix_timestamp_ms() as i64,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        update.success = match self.periphery.clone_repo(&server, &build).await {
            Ok(clone_logs) => {
                update.logs.extend(clone_logs);
                true
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("clone repo", format!("{e:#?}")));
                false
            }
        };

        update.status = UpdateStatus::Complete;
        update.end_ts = Some(unix_timestamp_ms() as i64);

        self.update_update(update.clone()).await?;

        Ok(update)
    }
}
