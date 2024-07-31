import { createContext, ReactNode, useContext } from "react";

interface User {
  firstName: string;
  lastName: string;
}

interface AuthContextType {
  user: User | null | undefined;
  login: () => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType>({
  user: undefined,
  login: () => {},
  logout: () => {},
});

export const useAuth = (): AuthContextType => {
  const ctx = useContext(AuthContext);
  if (ctx.user) {
    return ctx;
  }
  return ctx;
};

const initiateAuthContext = () => ({
  user: undefined,
  login: () => (document.location.href = "/api/v0/auth/login"),
  logout: () => (document.location.href = "/api/v0/auth/logout"),
});

export const AuthProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const authContext = initiateAuthContext();
  return (
    <AuthContext.Provider value={authContext}>{children}</AuthContext.Provider>
  );
};
