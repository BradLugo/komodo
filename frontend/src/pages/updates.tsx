import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { AddTags, ResourceTags } from "@components/tags";
import { UpdatesTable } from "@components/updates/table";
import { useRead, useResourceParamType, useSetTitle } from "@lib/hooks";
import { UsableResource } from "@types";
import { useParams } from "react-router-dom";

export const Updates = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;
  if (type && id) {
    return <ResourceUpdates type={type} id={id} />;
  } else {
    return <AllUpdates />;
  }
};

const AllUpdates = () => {
  useSetTitle("Updates");
  const updates = useRead("ListUpdates", {}).data;
  return (
    <Page title="Updates">
      <UpdatesTable updates={updates?.updates ?? []} />
    </Page>
  );
};

const ResourceUpdates = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const name = useRead(`List${type}s`, {}).data?.find((r) => r.id === id)?.name;
  useSetTitle(name && `${name} | Updates`);
  const updates = useRead("ListUpdates", {
    query: {
      "target.type": type,
      "target.id": id,
    },
  }).data;
  const Components = ResourceComponents[type];
  return (
    <Page
      title={<Components.Name id={id} />}
      titleRight={
        <div className="flex gap-2">
          <ResourceTags target={{ id, type }} click_to_delete />
          <AddTags target={{ id, type }} />
        </div>
      }
    >
      <UpdatesTable updates={updates?.updates ?? []} />
    </Page>
  );
};