import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { MAX_PAGE_WIDTH } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import { combineClasses, getId } from "../../util/helpers";
import NotFound from "../NotFound";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server2: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const { user } = useUser();
  // const userCanUpdate = () =>
  //   user().admin ||
  //   server()!.server.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <Show when={server()} fallback={<NotFound type="server" />}>
      <ActionStateProvider>
        <Grid
          style={{
            width: "100vw",
            "max-width": `${MAX_PAGE_WIDTH}px`,
            "box-sizing": "border-box",
          }}
        >
          <Grid style={{ width: "100%" }} gridTemplateColumns="1fr 1fr">
            <Grid>
              <Header />
              <Actions />
            </Grid>
            <Updates />
          </Grid>
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

const Server: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const { isSemiMobile } = useAppDimensions();
  const { user } = useUser();
  const userCanUpdate = () =>
    user().admin ||
    server()!.server.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <Show when={server()} fallback={<NotFound type="server" />}>
      <ActionStateProvider>
        <Grid class={combineClasses("content")}>
          {/* left / actions */}
          <Grid class="left-content">
            <Header />
            <Actions />
            <Show when={!isSemiMobile() && userCanUpdate()}>
              <Updates />
            </Show>
          </Grid>
          {/* right / tabs */}
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Server2;
