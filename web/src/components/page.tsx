import { AuthProvider } from "../utility/auth";
import { Header } from "./header";

export const Page: React.FC = () => {
  return (
    <AuthProvider>
      <Header />
    </AuthProvider>
  );
};
