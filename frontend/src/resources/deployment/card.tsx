import { useRead } from "@hooks";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Link } from "react-router-dom";
import { DeploymentInfo, DeploymentStatusIcon } from "./util";

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between">
          <div>
            <CardTitle>{deployment.name}</CardTitle>
            <CardDescription>
              {deployment.status ?? "not deployed"}
            </CardDescription>
          </div>
          <DeploymentStatusIcon deploymentId={id} />
        </CardHeader>
        <CardContent>
          <DeploymentInfo deploymentId={id} />
        </CardContent>
      </Card>
    </Link>
  );
};
