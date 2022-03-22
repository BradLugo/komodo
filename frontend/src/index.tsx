/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./components/App";
import Client from "./util/client";
import makeNotifications from "./components/util/notification/Notifications";
import { UserProvider } from "./state/UserProvider";
import { WidthProvider } from "./state/WidthProvider";
import LoginGuard from "./components/login/LoginGuard";
import { AppStateProvider } from "./state/StateProvider";

export const URL = "http://localhost:9000";
export const WS_URL = "ws://localhost:9000/ws";
export const client = new Client(URL);

export const { Notifications, pushNotification } = makeNotifications();

render(
  () => [
    <WidthProvider>
      <UserProvider>
        <LoginGuard>
          <AppStateProvider>
            <App />
          </AppStateProvider>
        </LoginGuard>
      </UserProvider>
    </WidthProvider>,
    <Notifications />,
  ],
  document.getElementById("root") as HTMLElement
);
