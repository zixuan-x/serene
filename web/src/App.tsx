import { Button } from "@nextui-org/react";
import "./App.css";

function App() {
  return (
    <>
      <div
        style={{
          width: "30%",
          height: "100vh",
          position: "fixed",
          left: 0,
          top: 0,
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        Menu
      </div>
      <div>
        <Button>Login</Button>
      </div>
    </>
  );
}

export default App;
