import { Outlet } from "react-router-dom";

export default function Layout() {
  return (
    <>
      <div className="container mx-auto p-3">
        <Outlet />
      </div>
    </>
  );
}
