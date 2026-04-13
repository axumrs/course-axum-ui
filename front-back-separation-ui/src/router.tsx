import { createHashRouter } from "react-router-dom";
import ListUser from "./pages/ListUser";
import FindUser from "./pages/FindUser";
import Layout from "./components/Layout";

const router = createHashRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <ListUser />,
      },
      {
        path: ":id",
        element: <FindUser />,
      },
    ],
  },
]);

export default router;
