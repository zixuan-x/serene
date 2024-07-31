import { UserIcon } from "./usericon";
import { Layout } from "antd";
const { Header: AntHeader } = Layout;

export const Header: React.FC = () => {
  return (
    <AntHeader
      style={{
        display: "flex",
        justifyContent: "flex-end",
        background: "white",
      }}
    >
      <UserIcon />
    </AntHeader>
  );
};
