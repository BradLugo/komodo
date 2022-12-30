import { Component, JSX } from "solid-js";
import { combineClasses, filterOutFromObj } from "../../../util/helpers";
import s from "./layout.module.scss";

const Grid: Component<
  {
    gap?: string | number;
    placeItems?: string;
    style?: JSX.CSSProperties;
    class?: string;
  } & JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  return (
    <div
      {...filterOutFromObj(p, ["gap", "placeItems", "style", "class"])}
      class={combineClasses(s.Grid, p.class)}
      style={{
        gap: p.gap,
        "place-items": p.placeItems,
        ...(p.style as any),
      }}
    >
      {p.children}
    </div>
  );
};

export default Grid;
