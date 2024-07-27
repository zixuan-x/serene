import React from "react";
import {
  LaptopOutlined,
  NotificationOutlined,
  UserOutlined,
} from "@ant-design/icons";
import type { MenuProps } from "antd";
import { Breadcrumb, Button, Layout, Menu, theme } from "antd";

const { Header, Content, Footer, Sider } = Layout;
const App: React.FC = () => {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  return (
    <Layout>
      <Header style={{ display: "flex", alignItems: "center" }}>
        <Button>User Profile</Button>
      </Header>
      <Content style={{ padding: "0 48px" }}>
        <Breadcrumb style={{ margin: "16px 0" }}>
          <Breadcrumb.Item>Home</Breadcrumb.Item>
          <Breadcrumb.Item>List</Breadcrumb.Item>
          <Breadcrumb.Item>App</Breadcrumb.Item>
        </Breadcrumb>
        <Layout
          style={{
            padding: "24px 0",
            background: colorBgContainer,
            borderRadius: borderRadiusLG,
            overflow: "auto",
          }}
        >
          <Sider style={{ background: colorBgContainer }} width={200}>
            Side Menu
          </Sider>
          <Content
            style={{ padding: "0 24px", minHeight: 280, overflow: "auto" }}
          >
            Content
          </Content>
        </Layout>
      </Content>
      <Footer style={{ textAlign: "center", overflow: "auto" }}>
        <Button>Project</Button>
        <Button>Today</Button>
      </Footer>
    </Layout>
  );
};

export default App;
