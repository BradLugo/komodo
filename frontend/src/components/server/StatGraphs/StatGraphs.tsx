import { Component, lazy } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useToggle } from "../../../util/hooks";
import Icon from "../../util/Icon";
import CenterMenu from "../../util/menu/CenterMenu";

const Graphs = lazy(() => import("./Graphs"))

const StatGraphs: Component<{ id: string }> = (p) => {
  const { servers } = useAppState();
  const [show, toggleShow] = useToggle();
  const name = () => servers.get(p.id)?.name;
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target={<Icon type="timeline-line-chart" width="0.85rem" />}
      targetClass="blue"
      content={<Graphs id={p.id} />}
      title={`${name()} stats`}
    />
  );
};

export default StatGraphs;