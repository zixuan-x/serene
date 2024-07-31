import { Layout } from "antd";
import { AuthProvider, useAuth } from "../utility/auth";
import { Header } from "../components/header";
export const Home: React.FC = () => {
  const ctx = useAuth();
  return (
    <Layout>
      <AuthProvider>
        <Header />
        {ctx.user ? <>{ctx.user}</> : <>Please Login</>}
      </AuthProvider>
    </Layout>
  );
};
