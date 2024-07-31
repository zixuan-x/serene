import { SmileOutlined, UserOutlined } from "@ant-design/icons";
import { Avatar, Dropdown, Space } from "antd";
import type { MenuProps } from "antd";
import { useAuth } from "../utility/auth";

export const UserIcon: React.FC = () => {
  const { user, login, logout } = useAuth();

  const items: MenuProps["items"] = user
    ? [
        {
          key: "2",
          label: (
            <a
              // target="_blank"
              // rel="noopener noreferrer"
              // href="https://www.aliyun.com"
            >
              Logout
            </a>
          ),
          icon: <SmileOutlined />,
          onClick: logout,
          // disabled: true,
        },
      ]
    : [
        {
          key: "1",
          label: (
            <a
              // target="_blank"
              // rel="noopener noreferrer"
              // href="https://www.antgroup.com"
            >
              Login
            </a>
          ),
          icon: <SmileOutlined />,
          onClick: login,
        },
      ];
  return (
    <Dropdown
      menu={{ items }}
      //   placement="topLeft" // Add this line to set the placement to top right
    >
      <a onClick={(e) => e.preventDefault()}>
        <Space
          direction="vertical"
          style={{ display: "flex", alignItems: "center" }}
        >
          <Avatar size="large" icon={<UserOutlined />} shape="square" />
        </Space>
      </a>
    </Dropdown>
  );
};
