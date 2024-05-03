use std::{collections::HashSet, str::FromStr};

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{self, *},
  entities::{
    builder::{Builder, BuilderConfig, BuilderListItem},
    permission::PermissionLevel,
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::{db_client, State},
};

#[async_trait]
impl Resolve<GetBuilder, User> for State {
  async fn resolve(
    &self,
    GetBuilder { builder }: GetBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    Builder::get_resource_check_permissions(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListBuilders, User> for State {
  async fn resolve(
    &self,
    ListBuilders { query }: ListBuilders,
    user: User,
  ) -> anyhow::Result<Vec<BuilderListItem>> {
    Builder::list_resource_list_items_for_user(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetBuildersSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildersSummary {}: GetBuildersSummary,
    user: User,
  ) -> anyhow::Result<GetBuildersSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Builder,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };
    let total = db_client()
      .await
      .builders
      .count_documents(query, None)
      .await
      .context("failed to count all builder documents")?;
    let res = GetBuildersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetBuilderAvailableAccounts, User> for State {
  async fn resolve(
    &self,
    GetBuilderAvailableAccounts { builder }: GetBuilderAvailableAccounts,
    user: User,
  ) -> anyhow::Result<GetBuilderAvailableAccountsResponse> {
    let builder = Builder::get_resource_check_permissions(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let (github, docker) = match builder.config {
      BuilderConfig::Aws(config) => {
        (config.github_accounts, config.docker_accounts)
      }
      BuilderConfig::Server(config) => {
        let res = self
          .resolve(
            read::GetAvailableAccounts {
              server: config.server_id,
            },
            user,
          )
          .await?;
        (res.github, res.docker)
      }
    };

    let mut github_set = HashSet::<String>::new();

    github_set.extend(core_config().github_accounts.keys().cloned());
    github_set.extend(github);

    let mut github = github_set.into_iter().collect::<Vec<_>>();
    github.sort();

    let mut docker_set = HashSet::<String>::new();

    docker_set.extend(core_config().docker_accounts.keys().cloned());
    docker_set.extend(docker);

    let mut docker = docker_set.into_iter().collect::<Vec<_>>();
    docker.sort();

    Ok(GetBuilderAvailableAccountsResponse { github, docker })
  }
}
