import { createBrowserRouter } from "react-router-dom";
// import { Page } from "../components/page";
import { Home } from "./home";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
    // children: [{ index: true, element: <Home /> }],
  },
]);
